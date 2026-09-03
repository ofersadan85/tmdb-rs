use super::{Collection, Company, Episode, Keyword, Movie, Person, Season, Tv};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// Search query parameters used across several TMDB search endpoints.
#[derive(Debug, Default, Serialize)]
pub struct SearchQueryCommon {
    pub(crate) query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page: Option<u32>,
}

pub trait SearchQuery {
    fn simple(query: String) -> Self;
    fn page(&self) -> Option<u32>;
    fn set_page(&mut self, page: u32);
}

impl SearchQuery for SearchQueryCommon {
    fn simple(query: String) -> Self {
        Self { query, page: None }
    }

    fn page(&self) -> Option<u32> {
        self.page
    }

    fn set_page(&mut self, page: u32) {
        self.page = Some(page);
    }
}

macro_rules! impl_query_simple {
    ($ty: ty) => {
        impl SearchQuery for $ty {
            #[allow(clippy::needless_update)]
            fn simple(query: String) -> Self {
                Self {
                    common: SearchQueryCommon::simple(query),
                    ..Default::default()
                }
            }

            fn page(&self) -> Option<u32> {
                self.common.page
            }

            fn set_page(&mut self, page: u32) {
                self.common.set_page(page);
            }
        }
    };
}

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryCollection {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon,
}

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryCompany {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon,
}

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryKeyword {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon,
}

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryMovie {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon,
    #[serde(flatten)]
    pub(crate) extra: SearchQueryExtra,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) primary_release_year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) region: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryMulti {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon,
    #[serde(flatten)]
    pub(crate) extra: SearchQueryExtra,
}

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryPerson {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon,
    #[serde(flatten)]
    pub(crate) extra: SearchQueryExtra,
}

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryTv {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon,
    #[serde(flatten)]
    pub(crate) extra: SearchQueryExtra,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) first_air_date_year: Option<u16>,
}

impl_query_simple!(SearchQueryCollection);
impl_query_simple!(SearchQueryCompany);
impl_query_simple!(SearchQueryKeyword);
impl_query_simple!(SearchQueryMovie);
impl_query_simple!(SearchQueryMulti);
impl_query_simple!(SearchQueryPerson);
impl_query_simple!(SearchQueryTv);

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryExtra {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) include_adult: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) language: Option<String>,
}

#[async_trait::async_trait]
pub trait Searchable: Sized {
    type Query: serde::Serialize + SearchQuery + Default + Send;
    const SEARCH_PATH: &'static str;

    async fn search(
        client: &crate::TmdbClient,
        base_url: &str,
        query: Self::Query,
    ) -> Result<SearchResults<Self, Self::Query>, reqwest::Error>
    where
        Self: DeserializeOwned,
    {
        let mut results: SearchResults<Self, Self::Query> = client
            .client
            .get(format!("{}/{}", base_url, Self::SEARCH_PATH))
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        results.query = Some(query);
        Ok(results)
    }

    async fn search_simple(
        client: &crate::TmdbClient,
        base_url: &str,
        query: impl Into<String> + Send,
    ) -> Result<SearchResults<Self, Self::Query>, reqwest::Error>
    where
        Self: DeserializeOwned,
    {
        Self::search(client, base_url, Self::Query::simple(query.into())).await
    }
}

impl Searchable for Collection {
    type Query = SearchQueryCollection;
    const SEARCH_PATH: &'static str = "search/collection";
}

impl Searchable for Company {
    type Query = SearchQueryCompany;
    const SEARCH_PATH: &'static str = "search/company";
}
impl Searchable for Keyword {
    type Query = SearchQueryKeyword;
    const SEARCH_PATH: &'static str = "search/keyword";
}

impl Searchable for Movie {
    type Query = SearchQueryMovie;
    const SEARCH_PATH: &'static str = "search/movie";
}

impl Searchable for MultiSearch {
    type Query = SearchQueryMulti;
    const SEARCH_PATH: &'static str = "search/multi";
}

impl Searchable for Person {
    type Query = SearchQueryPerson;
    const SEARCH_PATH: &'static str = "search/person";
}

impl Searchable for Tv {
    type Query = SearchQueryTv;
    const SEARCH_PATH: &'static str = "search/tv";
}

/// Generic search response used across several TMDB search endpoints.
#[derive(Debug, Deserialize)]
pub struct SearchResults<T, Q> {
    pub page: u32,
    pub total_results: u32,
    pub total_pages: u32,
    pub results: Vec<T>,
    #[serde(skip)]
    pub(crate) query: Option<Q>,
}

impl<T, Q> SearchResults<T, Q>
where
    Q: SearchQuery + Clone,
{
    pub fn query_next_page(&self) -> Option<Q> {
        if let Some(page) = self.query.as_ref().and_then(SearchQuery::page)
            && page < self.total_pages
            && let Some(query) = self.query.as_ref()
        {
            let mut next_query = query.clone();
            next_query.set_page(page + 1);
            return Some(next_query);
        }
        None
    }

    pub fn query_previous_page(&self) -> Option<Q> {
        if let Some(page) = self.query.as_ref().and_then(SearchQuery::page)
            && page > 1
            && let Some(query) = self.query.as_ref()
        {
            let mut previous_query = query.clone();
            previous_query.set_page(page - 1);
            return Some(previous_query);
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "media_type", rename_all = "snake_case")]
#[expect(clippy::large_enum_variant)]
pub enum MultiSearch {
    Collection(Collection),
    Company(Company),
    Keyword(Keyword),
    Movie(Movie),
    Person(Person),
    Tv(Tv),
}

/// External sources for finding items by their external IDs.
#[derive(Serialize)]
pub enum ExternalSourceId {
    #[serde(rename = "imdb_id")]
    Imdb,
    #[serde(rename = "facebook_id")]
    Facebook,
    #[serde(rename = "instagram_id")]
    Instagram,
    #[serde(rename = "tvdb_id")]
    Tvdb,
    #[serde(rename = "tiktok_id")]
    TikTok,
    #[serde(rename = "twitter_id")]
    Twitter,
    #[serde(rename = "wikidata_id")]
    Wikidata,
    #[serde(rename = "youtube_id")]
    YouTube,
}

/// Response returned when finding items by their external IDs.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct FindResponse {
    #[serde(rename = "movie_results")]
    pub(crate) movies: Vec<Movie>,
    #[serde(rename = "person_results")]
    pub(crate) people: Vec<Person>,
    #[serde(rename = "tv_results")]
    pub(crate) tv: Vec<Tv>,
    #[serde(rename = "tv_episode_results")]
    pub(crate) episodes: Vec<Episode>,
    #[serde(rename = "tv_season_results")]
    pub(crate) seasons: Vec<Season>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TmdbClient;

    fn client() -> TmdbClient {
        dotenvy::dotenv().ok();
        TmdbClient::from_env("TMDB_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn external_id() {
        let response = client()
            .find_by_external_id("tt33764258", ExternalSourceId::Imdb, None)
            .await
            .unwrap();
        assert!(!response.movies.is_empty(), "{response:#?}");
        assert_eq!(response.movies[0].title, "The Odyssey");
    }

    #[tokio::test]
    async fn movie() {
        let response = client().search_simple::<Movie>("Inception").await.unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
        assert_eq!(response.results[0].id, 27205);
        assert_eq!(response.results[0].title, "Inception");
    }

    #[tokio::test]
    async fn tv() {
        dotenvy::dotenv().ok();
        let response = client()
            .search_simple::<Tv>("Game of Thrones")
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
        assert_eq!(response.results[0].id, 1399);
        assert_eq!(response.results[0].name, "Game of Thrones");
    }

    #[tokio::test]
    async fn person() {
        let response = client()
            .search_simple::<Person>("Keanu Reeves")
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
        assert_eq!(response.results[0].id, 6384);
        assert_eq!(response.results[0].name, "Keanu Reeves");
    }

    #[tokio::test]
    async fn collection() {
        let response = client()
            .search_simple::<Collection>("Star Wars")
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
        assert_eq!(response.results[0].id, 10);
        assert_eq!(response.results[0].name, "Star Wars Collection");
    }

    #[tokio::test]
    async fn company() {
        let response = client().search_simple::<Company>("Pixar").await.unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
        assert_eq!(response.results[0].id, 3);
        assert_eq!(response.results[0].name, "Pixar");
    }

    #[tokio::test]
    async fn keyword() {
        let response = client().search_simple::<Keyword>("space").await.unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
        assert_eq!(response.results[0].id, 9882);
        assert_eq!(response.results[0].name, "space");
    }

    #[tokio::test]
    async fn multi() {
        let response = client()
            .search_simple::<MultiSearch>("Avatar")
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
        assert_ne!(response.total_pages, 1);
    }
}
