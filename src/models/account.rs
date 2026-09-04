use super::{
    Episode, Movie, Tv,
    search::{SearchQuery, SearchResults},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Default)]
enum SortBy {
    #[default]
    CreatedAtAscending,
    CreatedAtDescending,
}

impl Serialize for SortBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::CreatedAtAscending => serializer.serialize_str("created_at.asc"),
            Self::CreatedAtDescending => serializer.serialize_str("created_at.desc"),
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(default)]
pub struct AccountQuery {
    #[serde(skip)] // Path parameter, not part of the query filter
    account_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "String::is_empty")]
    session: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<SortBy>,
}

impl super::search::SearchQuery for AccountQuery {
    fn page(&self) -> Option<u32> {
        self.page
    }

    fn set_page(&mut self, page: u32) {
        self.page = Some(page);
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Gravatar {
    pub hash: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct TmdbAvatar {
    pub avatar_path: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Avatar {
    pub gravatar: Option<Gravatar>,
    pub tmdb: Option<TmdbAvatar>,
}

/// Public account details on TMDB.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct AccountDetails {
    pub id: u64,
    pub name: String,
    pub username: String,
    #[serde(rename = "iso_639_1")]
    pub language: String,
    #[serde(rename = "iso_3166_1")]
    pub country: String,
    pub avatar: Option<Avatar>,
    pub include_adult: bool,
}

pub trait OnAccountLists {
    const ENDPOINT: &'static str;
}

impl OnAccountLists for Movie {
    const ENDPOINT: &'static str = "movies";
}

impl OnAccountLists for Tv {
    const ENDPOINT: &'static str = "tv";
}

impl OnAccountLists for Episode {
    const ENDPOINT: &'static str = "tv/episodes";
}

pub trait OnNonRatedAccountLists: OnAccountLists {}
impl OnNonRatedAccountLists for Movie {}
impl OnNonRatedAccountLists for Tv {}

enum AccountListKind {
    Favorite,
    Rated,
    Watchlist,
}

impl std::fmt::Display for AccountListKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Favorite => "favorite",
            Self::Rated => "rated",
            Self::Watchlist => "watchlist",
        };
        write!(f, "{s}")
    }
}

impl AccountDetails {
    /// Get account details.
    ///
    /// This will always return the calling user's details.
    /// Use [`Self::get_with_session`] to retrieve account details for a specific session.
    pub async fn get(client: &crate::TmdbClient) -> Result<Self, reqwest::Error> {
        Self::get_with_session(client, 0, None).await
    }

    /// Get account details by account ID and session ID
    ///
    /// Calling this function with [`None`] as a session ID is equivalent to using
    /// [`Self::get`] and will only return the calling user's details, regardless
    /// of the provided account ID.
    pub async fn get_with_session(
        client: &crate::TmdbClient,
        id: u64,
        session: Option<&str>,
    ) -> Result<Self, reqwest::Error> {
        let url = format!("{}/account/{id}", client.base_url);
        let mut req = client.client.get(&url);
        if let Some(session) = session {
            req = req.query(&[("session_id", session)]);
        }
        req.send().await?.error_for_status()?.json().await
    }

    async fn get_account_list<T>(
        &self,
        client: &crate::TmdbClient,
        query: AccountQuery,
        list_type: AccountListKind,
    ) -> Result<SearchResults<T, AccountQuery>, reqwest::Error>
    where
        T: OnAccountLists + DeserializeOwned,
    {
        let url = format!(
            "{}/account/{}/{}/{}",
            client.base_url,
            self.id,
            list_type,
            T::ENDPOINT
        );
        let mut results: SearchResults<T, AccountQuery> = client
            .client
            .get(&url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        results.query = Some(query);
        Ok(results)
    }

    pub async fn favorites<T>(
        &self,
        client: &crate::TmdbClient,
        query: AccountQuery,
    ) -> Result<SearchResults<T, AccountQuery>, reqwest::Error>
    where
        T: OnNonRatedAccountLists + DeserializeOwned,
    {
        self.get_account_list(client, query, AccountListKind::Favorite)
            .await
    }

    pub async fn rated<T>(
        &self,
        client: &crate::TmdbClient,
        query: AccountQuery,
    ) -> Result<SearchResults<T, AccountQuery>, reqwest::Error>
    where
        T: OnAccountLists + DeserializeOwned,
    {
        self.get_account_list(client, query, AccountListKind::Rated)
            .await
    }

    pub async fn watchlist<T>(
        &self,
        client: &crate::TmdbClient,
        query: AccountQuery,
    ) -> Result<SearchResults<T, AccountQuery>, reqwest::Error>
    where
        T: OnNonRatedAccountLists + DeserializeOwned,
    {
        self.get_account_list(client, query, AccountListKind::Watchlist)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    fn client() -> crate::TmdbClient {
        dotenvy::dotenv().ok();
        crate::TmdbClient::from_env("TMDB_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn get_account_details() {
        let account_details = AccountDetails::get(&client()).await.unwrap();
        assert_eq!(account_details.id, 21_258_468);
        assert_eq!(account_details.username, "ofersadan");
    }

    #[tokio::test]
    async fn movies_favorites() {
        let response = AccountDetails::default()
            .favorites::<Movie>(&client(), AccountQuery::default())
            .await
            .unwrap();
        assert_eq!(response.page, 1, "{response:#?}");
    }

    #[tokio::test]
    async fn movies_rated() {
        let response = AccountDetails::default()
            .rated::<Movie>(&client(), AccountQuery::default())
            .await
            .unwrap();
        assert_eq!(response.page, 1, "{response:#?}");
    }

    #[tokio::test]
    async fn movies_watchlist() {
        let response = AccountDetails::default()
            .watchlist::<Movie>(&client(), AccountQuery::default())
            .await
            .unwrap();
        assert_eq!(response.page, 1, "{response:#?}");
    }

    #[tokio::test]
    async fn tv_favorites() {
        let response = AccountDetails::default()
            .favorites::<Tv>(&client(), AccountQuery::default())
            .await
            .unwrap();
        assert_eq!(response.page, 1, "{response:#?}");
    }

    #[tokio::test]
    async fn tv_rated() {
        let response = AccountDetails::default()
            .rated::<Tv>(&client(), AccountQuery::default())
            .await
            .unwrap();
        assert_eq!(response.page, 1, "{response:#?}");
    }

    #[tokio::test]
    async fn tv_watchlist() {
        let response = AccountDetails::default()
            .watchlist::<Tv>(&client(), AccountQuery::default())
            .await
            .unwrap();
        assert_eq!(response.page, 1, "{response:#?}");
    }

    #[tokio::test]
    async fn episodes_rated() {
        let response = AccountDetails::default()
            .rated::<Episode>(&client(), AccountQuery::default())
            .await
            .unwrap();
        assert_eq!(response.page, 1, "{response:#?}");
    }
}
