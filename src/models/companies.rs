use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Company {
    pub description: Option<String>,
    pub headquarters: Option<String>,
    pub homepage: Option<String>,
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: Option<String>,
    #[serde(rename = "parent_company")]
    pub parent: Option<Box<Self>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Network {
    #[serde(flatten)]
    company: Company,
}

pub enum CompanyKind {
    Company,
    Network,
}

impl std::fmt::Display for CompanyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Company => write!(f, "company"),
            Self::Network => write!(f, "network"),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct AlternativeName {
    pub name: String,
    #[serde(rename = "type")]
    pub name_type: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct AlternativeNamesResult {
    pub id: u64,
    pub results: Vec<AlternativeName>,
}

impl AlternativeNamesResult {
    pub async fn from_id(
        client: &crate::TmdbClient,
        id: u64,
        kind: CompanyKind,
    ) -> Result<Self, reqwest::Error> {
        let url = format!("{}/{kind}/{id}/alternative_names", client.base_url);
        client.client.get(&url).send().await?.json().await
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct ImagesResult {
    pub id: u64,
    pub logos: Vec<Image>,
}

impl ImagesResult {
    /// Retrieves the images of a company or network by its ID.
    #[doc = crate::tmdb_api_error_docs!()]
    pub async fn from_id(
        client: &crate::TmdbClient,
        id: u64,
        kind: CompanyKind,
    ) -> Result<Self, reqwest::Error> {
        let url = format!("{}/{kind}/{id}/images", client.base_url);
        client.client.get(&url).send().await?.json().await
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Image {
    pub file_path: String,
    pub width: u32,
    pub height: u32,
    pub iso_639_1: Option<String>,
}

impl Company {
    /// Retrieves the details of a company by its ID.
    #[doc = crate::tmdb_api_error_docs!()]
    pub async fn details(client: &crate::TmdbClient, id: u64) -> Result<Self, reqwest::Error> {
        let url = format!("{}/company/{id}", client.base_url);
        client.client.get(&url).send().await?.json().await
    }

    /// Retrieves the alternative names of the company.
    #[doc = crate::tmdb_api_error_docs!()]
    pub async fn alternative_names(
        &self,
        client: &crate::TmdbClient,
    ) -> Result<AlternativeNamesResult, reqwest::Error> {
        AlternativeNamesResult::from_id(client, self.id, CompanyKind::Company).await
    }

    /// Retrieves the images of the company.
    #[doc = crate::tmdb_api_error_docs!()]
    pub async fn images(&self, client: &crate::TmdbClient) -> Result<ImagesResult, reqwest::Error> {
        ImagesResult::from_id(client, self.id, CompanyKind::Company).await
    }
}

impl Network {
    /// Retrieves the details of a network by its ID.
    #[doc = crate::tmdb_api_error_docs!()]
    pub async fn details(client: &crate::TmdbClient, id: u64) -> Result<Self, reqwest::Error> {
        let url = format!("{}/network/{id}", client.base_url);
        client.client.get(&url).send().await?.json().await
    }

    /// Retrieves the alternative names of the network.
    #[doc = crate::tmdb_api_error_docs!()]
    pub async fn alternative_names(
        &self,
        client: &crate::TmdbClient,
    ) -> Result<AlternativeNamesResult, reqwest::Error> {
        AlternativeNamesResult::from_id(client, self.id, CompanyKind::Network).await
    }

    /// Retrieves the images of the network.
    #[doc = crate::tmdb_api_error_docs!()]
    pub async fn images(&self, client: &crate::TmdbClient) -> Result<ImagesResult, reqwest::Error> {
        ImagesResult::from_id(client, self.id, CompanyKind::Network).await
    }
}

impl std::ops::Deref for Network {
    type Target = Company;

    fn deref(&self) -> &Self::Target {
        &self.company
    }
}

impl std::ops::DerefMut for Network {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.company
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn client() -> TmdbClient {
        dotenvy::dotenv().ok();
        TmdbClient::from_env("TMDB_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn company() {
        let client = client();
        let company = Company::details(&client, 2).await.unwrap();
        assert_eq!(company.id, 2);
        assert_eq!(company.name, "Walt Disney Pictures");
        let alternatives = company.alternative_names(&client).await.unwrap();
        assert_eq!(alternatives.id, company.id);
        assert!(!alternatives.results.is_empty());
        let images = company.images(&client).await.unwrap();
        assert_eq!(images.id, company.id);
        assert!(!images.logos.is_empty());
    }

    #[tokio::test]
    async fn network() {
        let client = client();
        let network = Network::details(&client, 49).await.unwrap();
        assert_eq!(network.id, 49);
        assert_eq!(network.name, "HBO");
        let alternatives = network.alternative_names(&client).await.unwrap();
        assert_eq!(alternatives.id, network.id);
        assert!(!alternatives.results.is_empty());
        let images = network.images(&client).await.unwrap();
        assert_eq!(images.id, network.id);
        assert!(!images.logos.is_empty());
    }
}
