use crate::models::MediaType;

use super::{
    Episode, Movie, TmdbResponse, Tv,
    lists::AddToListRequest,
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

pub trait OnNonRatedAccountLists: OnAccountLists {
    const MEDIA_TYPE: MediaType;

    /// Returns the ID of the item.
    fn get_id(&self) -> u64;

    /// Sets the favorite status for the TV show on the account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request to the TMDB API fails.
    async fn set_favorite(
        &self,
        client: &crate::TmdbClient,
        account: &AccountDetails,
        favorite: bool,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        account
            .special_list_toggle(
                client,
                AccountListKind::Favorite,
                Self::MEDIA_TYPE,
                self.get_id(),
                favorite,
            )
            .await
    }

    /// Sets the watchlist status for the TV show on the account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request to the TMDB API fails.
    async fn set_watchlist(
        &self,
        client: &crate::TmdbClient,
        account: &AccountDetails,
        watchlist: bool,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        account
            .special_list_toggle(
                client,
                AccountListKind::Watchlist,
                Self::MEDIA_TYPE,
                self.get_id(),
                watchlist,
            )
            .await
    }
}

impl OnNonRatedAccountLists for Movie {
    const MEDIA_TYPE: MediaType = MediaType::Movie;

    fn get_id(&self) -> u64 {
        self.id
    }
}

impl OnNonRatedAccountLists for Tv {
    const MEDIA_TYPE: MediaType = MediaType::Tv;

    fn get_id(&self) -> u64 {
        self.id
    }
}

pub enum AccountListKind {
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

    async fn special_list_toggle(
        &self,
        client: &crate::TmdbClient,
        list_type: AccountListKind,
        media_type: MediaType,
        media_id: u64,
        on_list: bool,
    ) -> Result<TmdbResponse, reqwest::Error> {
        let url = format!("{}/account/{}/{list_type}", client.base_url, self.id);
        let mut req = AddToListRequest::new(media_type, media_id);
        match list_type {
            AccountListKind::Favorite => req.favorite = Some(on_list),
            AccountListKind::Watchlist => req.watchlist = Some(on_list),
            AccountListKind::Rated => unimplemented!("rated is not supported here"),
        }
        if matches!(media_type, MediaType::Person) {
            unimplemented!("person media type is not supported here");
        }
        client
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Adds an item to the account favorites.
    pub async fn favorite_add(
        &self,
        client: &crate::TmdbClient,
        media_type: MediaType,
        media_id: u64,
    ) -> Result<TmdbResponse, reqwest::Error> {
        self.special_list_toggle(
            client,
            AccountListKind::Favorite,
            media_type,
            media_id,
            true,
        )
        .await
    }

    /// Adds an item to the account watchlist.
    pub async fn watchlist_add(
        &self,
        client: &crate::TmdbClient,
        media_type: MediaType,
        media_id: u64,
    ) -> Result<TmdbResponse, reqwest::Error> {
        self.special_list_toggle(
            client,
            AccountListKind::Watchlist,
            media_type,
            media_id,
            true,
        )
        .await
    }

    /// Removes an item from the account favorites.
    pub async fn favorite_remove(
        &self,
        client: &crate::TmdbClient,
        media_type: MediaType,
        media_id: u64,
    ) -> Result<TmdbResponse, reqwest::Error> {
        self.special_list_toggle(
            client,
            AccountListKind::Favorite,
            media_type,
            media_id,
            false,
        )
        .await
    }

    /// Removes an item from the account watchlist.
    pub async fn watchlist_remove(
        &self,
        client: &crate::TmdbClient,
        media_type: MediaType,
        media_id: u64,
    ) -> Result<TmdbResponse, reqwest::Error> {
        self.special_list_toggle(
            client,
            AccountListKind::Watchlist,
            media_type,
            media_id,
            false,
        )
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

    #[tokio::test]
    async fn add_movie_to_favorites() {
        let client = client();
        let response = AccountDetails::default()
            .favorite_add(&client, MediaType::Movie, 550)
            .await
            .unwrap();
        assert!(response.success, "{response:#?}");
    }

    #[tokio::test]
    async fn add_movie_to_watchlist() {
        let client = client();
        let response = AccountDetails::default()
            .watchlist_add(&client, MediaType::Movie, 550)
            .await
            .unwrap();
        assert!(response.success, "{response:#?}");
    }

    #[tokio::test]
    async fn add_tv_to_favorites() {
        let client = client();
        let response = AccountDetails::default()
            .favorite_add(&client, MediaType::Tv, 1399)
            .await
            .unwrap();
        assert!(response.success, "{response:#?}");
    }

    #[tokio::test]
    async fn add_tv_to_watchlist() {
        let client = client();
        let response = AccountDetails::default()
            .watchlist_add(&client, MediaType::Tv, 1399)
            .await
            .unwrap();
        assert!(response.success, "{response:#?}");
    }

    #[tokio::test]
    async fn favorite_add_remove() {
        let client = client();
        let account = AccountDetails::default();
        let response = account
            .favorite_add(&client, MediaType::Movie, 11)
            .await
            .unwrap();
        assert!(response.success, "{response:#?}");
        let favorites = account
            .favorites::<Movie>(&client, AccountQuery::default())
            .await
            .unwrap();
        assert!(
            favorites.results.iter().any(|movie| movie.id == 11),
            "{favorites:#?}"
        );
        let response = account
            .favorite_remove(&client, MediaType::Movie, 11)
            .await
            .unwrap();
        assert!(response.success, "{response:#?}");
        let favorites = account
            .favorites::<Movie>(&client, AccountQuery::default())
            .await
            .unwrap();
        assert!(
            !favorites.results.iter().any(|movie| movie.id == 11),
            "{favorites:#?}"
        );
    }

    #[tokio::test]
    async fn watchlist_add_remove() {
        let client = client();
        let account = AccountDetails::default();
        let response = account
            .watchlist_add(&client, MediaType::Movie, 11)
            .await
            .unwrap();
        assert!(response.success, "{response:#?}");
        let watchlist = account
            .watchlist::<Movie>(&client, AccountQuery::default())
            .await
            .unwrap();
        assert!(
            watchlist.results.iter().any(|movie| movie.id == 11),
            "{watchlist:#?}"
        );
        let response = account
            .watchlist_remove(&client, MediaType::Movie, 11)
            .await
            .unwrap();
        assert!(response.success, "{response:#?}");
        let watchlist = account
            .watchlist::<Movie>(&client, AccountQuery::default())
            .await
            .unwrap();
        assert!(
            !watchlist.results.iter().any(|movie| movie.id == 11),
            "{watchlist:#?}"
        );
    }
}
