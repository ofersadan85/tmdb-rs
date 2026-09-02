use super::{
    collections::Collection,
    companies::Company,
    keywords::Keyword,
    movies::Movie,
    people::Person,
    tv::{Episode, Season, TV},
};
use serde::{Deserialize, Serialize};

/// Search query parameters used across several TMDB search endpoints.
#[derive(Debug, Serialize)]
pub struct SearchQueryCommon<'a> {
    pub(crate) query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page: Option<u32>,
}

impl<'a> SearchQueryCommon<'a> {
    pub(crate) const fn simple(query: &'a str) -> Self {
        Self { query, page: None }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SearchQueryExtra<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) include_adult: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) language: Option<&'a str>,
}

impl SearchQueryExtra<'_> {
    pub(crate) const fn new() -> Self {
        Self {
            include_adult: None,
            language: None,
        }
    }
}

/// TODO: This is incomplete
/// Represents a region for search queries.
#[derive(Debug, Serialize)]
pub enum Region {
    US,
}

#[derive(Debug, Serialize)]
pub struct SearchQueryCollection<'a> {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon<'a>,
}

#[derive(Debug, Serialize)]
pub struct SearchQueryCompany<'a> {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon<'a>,
}

#[derive(Debug, Serialize)]
pub struct SearchQueryKeyword<'a> {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon<'a>,
}

#[derive(Debug, Serialize)]
pub struct SearchQueryMovie<'a> {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon<'a>,
    #[serde(flatten)]
    pub(crate) extra: SearchQueryExtra<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) primary_release_year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) region: Option<Region>,
}

#[derive(Debug, Serialize)]
pub struct SearchQueryMulti<'a> {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon<'a>,
    #[serde(flatten)]
    pub(crate) extra: SearchQueryExtra<'a>,
}

#[derive(Debug, Serialize)]
pub struct SearchQueryPerson<'a> {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon<'a>,
    #[serde(flatten)]
    pub(crate) extra: SearchQueryExtra<'a>,
}

#[derive(Debug, Serialize)]
pub struct SearchQueryTv<'a> {
    #[serde(flatten)]
    pub(crate) common: SearchQueryCommon<'a>,
    #[serde(flatten)]
    pub(crate) extra: SearchQueryExtra<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) first_air_date_year: Option<u16>,
}

#[derive(Debug)]
pub enum SearchQuery<'a> {
    Collection(SearchQueryCollection<'a>),
    Company(SearchQueryCompany<'a>),
    Keyword(SearchQueryKeyword<'a>),
    Movie(SearchQueryMovie<'a>),
    Multi(SearchQueryMulti<'a>),
    Person(SearchQueryPerson<'a>),
    Tv(SearchQueryTv<'a>),
}

impl Serialize for SearchQuery<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // This manual implementation of `Serialize` is necessary because `serde_urlencoded`
        // does not support serializing enums directly.
        match self {
            SearchQuery::Collection(value) => value.serialize(serializer),
            SearchQuery::Company(value) => value.serialize(serializer),
            SearchQuery::Keyword(value) => value.serialize(serializer),
            SearchQuery::Movie(value) => value.serialize(serializer),
            SearchQuery::Multi(value) => value.serialize(serializer),
            SearchQuery::Person(value) => value.serialize(serializer),
            SearchQuery::Tv(value) => value.serialize(serializer),
        }
    }
}

impl<'a> SearchQuery<'a> {
    pub(crate) const fn simple_collection(query: &'a str) -> Self {
        Self::Collection(SearchQueryCollection {
            common: SearchQueryCommon::simple(query),
        })
    }

    pub(crate) const fn simple_company(query: &'a str) -> Self {
        Self::Company(SearchQueryCompany {
            common: SearchQueryCommon::simple(query),
        })
    }

    pub(crate) const fn simple_keyword(query: &'a str) -> Self {
        Self::Keyword(SearchQueryKeyword {
            common: SearchQueryCommon::simple(query),
        })
    }

    pub(crate) const fn simple_movie(query: &'a str) -> Self {
        Self::Movie(SearchQueryMovie {
            common: SearchQueryCommon::simple(query),
            extra: SearchQueryExtra::new(),
            year: None,
            primary_release_year: None,
            region: None,
        })
    }

    pub(crate) const fn simple_multi(query: &'a str) -> Self {
        Self::Multi(SearchQueryMulti {
            common: SearchQueryCommon::simple(query),
            extra: SearchQueryExtra::new(),
        })
    }

    pub(crate) const fn simple_person(query: &'a str) -> Self {
        Self::Person(SearchQueryPerson {
            common: SearchQueryCommon::simple(query),
            extra: SearchQueryExtra::new(),
        })
    }

    pub(crate) const fn simple_tv(query: &'a str) -> Self {
        Self::Tv(SearchQueryTv {
            common: SearchQueryCommon::simple(query),
            extra: SearchQueryExtra::new(),
            year: None,
            first_air_date_year: None,
        })
    }

    pub(crate) const fn uri(&self) -> &'static str {
        match self {
            SearchQuery::Collection(_) => "search/collection",
            SearchQuery::Company(_) => "search/company",
            SearchQuery::Keyword(_) => "search/keyword",
            SearchQuery::Movie(_) => "search/movie",
            SearchQuery::Multi(_) => "search/multi",
            SearchQuery::Person(_) => "search/person",
            SearchQuery::Tv(_) => "search/tv",
        }
    }
}

impl<'a> From<SearchQueryCollection<'a>> for SearchQuery<'a> {
    fn from(value: SearchQueryCollection<'a>) -> Self {
        SearchQuery::Collection(value)
    }
}

impl<'a> From<SearchQueryCompany<'a>> for SearchQuery<'a> {
    fn from(value: SearchQueryCompany<'a>) -> Self {
        SearchQuery::Company(value)
    }
}

impl<'a> From<SearchQueryKeyword<'a>> for SearchQuery<'a> {
    fn from(value: SearchQueryKeyword<'a>) -> Self {
        SearchQuery::Keyword(value)
    }
}

impl<'a> From<SearchQueryMovie<'a>> for SearchQuery<'a> {
    fn from(value: SearchQueryMovie<'a>) -> Self {
        SearchQuery::Movie(value)
    }
}

impl<'a> From<SearchQueryMulti<'a>> for SearchQuery<'a> {
    fn from(value: SearchQueryMulti<'a>) -> Self {
        SearchQuery::Multi(value)
    }
}

impl<'a> From<SearchQueryPerson<'a>> for SearchQuery<'a> {
    fn from(value: SearchQueryPerson<'a>) -> Self {
        SearchQuery::Person(value)
    }
}

impl<'a> From<SearchQueryTv<'a>> for SearchQuery<'a> {
    fn from(value: SearchQueryTv<'a>) -> Self {
        SearchQuery::Tv(value)
    }
}

/// Generic search response used across several TMDB search endpoints.
#[derive(Debug, Deserialize)]
pub struct SearchResults {
    pub page: u32,
    pub total_results: u32,
    pub total_pages: u32,
    pub results: Vec<SearchResult>,
}

/// Generic search response used across several TMDB search endpoints.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[expect(clippy::large_enum_variant)]
pub enum SearchResult {
    Collection(Collection),
    Company(Company),
    Keyword(Keyword),
    Movie(Movie),
    Person(Person),
    Tv(TV),
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
    pub(crate) tv: Vec<TV>,
    #[serde(rename = "tv_episode_results")]
    pub(crate) episodes: Vec<Episode>,
    #[serde(rename = "tv_season_results")]
    pub(crate) seasons: Vec<Season>,
}
