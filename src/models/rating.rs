use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct RatingRequest {
    /// Rating value must be a multiple of 0.5 between 0.5 and 10.0
    value: f32,
}

/// Trait for adding and removing ratings for an item
#[async_trait::async_trait]
pub trait Rating {
    fn rating_endpoint(&self) -> impl std::fmt::Display;

    /// Adds a rating for the item.
    ///
    /// Rating value must be a multiple of 0.5 between 0.5 and 10.0
    async fn rating_add(
        &self,
        client: &crate::TmdbClient,
        rating: f32,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        let url = format!("{}/{}/rating", client.base_url, self.rating_endpoint());
        client
            .client
            .post(&url)
            .json(&RatingRequest { value: rating })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Removes a rating for the item.
    ///
    /// Rating value must be a multiple of 0.5 between 0.5 and 10.0
    async fn rating_delete(
        &self,
        client: &crate::TmdbClient,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        let url = format!("{}/{}/rating", client.base_url, self.rating_endpoint());
        client
            .client
            .delete(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}

impl Rating for super::Movie {
    fn rating_endpoint(&self) -> impl std::fmt::Display {
        format!("movie/{}", self.id)
    }
}

impl Rating for super::Tv {
    fn rating_endpoint(&self) -> impl std::fmt::Display {
        format!("tv/{}", self.id)
    }
}

impl Rating for super::Episode {
    fn rating_endpoint(&self) -> impl std::fmt::Display {
        format!(
            "tv/{}/season/{}/episode/{}",
            self.show_id, self.season_number, self.number
        )
    }
}

impl crate::TmdbClient {
    /// Adds a rating for a movie.
    ///
    /// Rating value must be a multiple of 0.5 between 0.5 and 10.0
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request fails.
    pub async fn rating_add_movie(
        &self,
        id: u64,
        rating: f32,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        super::Movie {
            id,
            ..Default::default()
        }
        .rating_add(self, rating)
        .await
    }

    /// Removes a rating for a movie.
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request fails.
    pub async fn rating_delete_movie(
        &self,
        id: u64,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        super::Movie {
            id,
            ..Default::default()
        }
        .rating_delete(self)
        .await
    }

    /// Adds a rating for a TV show.
    ///
    /// Rating value must be a multiple of 0.5 between 0.5 and 10.0
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request fails.
    pub async fn rating_add_tv(
        &self,
        id: u64,
        rating: f32,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        super::Tv {
            id,
            ..Default::default()
        }
        .rating_add(self, rating)
        .await
    }

    /// Removes a rating for a TV show.
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request fails.
    pub async fn rating_delete_tv(&self, id: u64) -> Result<super::TmdbResponse, reqwest::Error> {
        super::Tv {
            id,
            ..Default::default()
        }
        .rating_delete(self)
        .await
    }

    /// Adds a rating for an episode.
    ///
    /// Rating value must be a multiple of 0.5 between 0.5 and 10.0
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request fails.
    pub async fn rating_add_episode(
        &self,
        show_id: u64,
        season_number: u64,
        episode_number: u64,
        rating: f32,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        super::Episode {
            show_id,
            season_number,
            number: episode_number,
            ..Default::default()
        }
        .rating_add(self, rating)
        .await
    }

    /// Removes a rating for an episode.
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request fails.
    pub async fn rating_delete_episode(
        &self,
        show_id: u64,
        season_number: u64,
        episode_number: u64,
    ) -> Result<super::TmdbResponse, reqwest::Error> {
        super::Episode {
            show_id,
            season_number,
            number: episode_number,
            ..Default::default()
        }
        .rating_delete(self)
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
    async fn movie() {
        // Movie ID 550 => Fight Club
        let client = client();
        let result = client.rating_delete_movie(550).await.unwrap();
        assert!(result.success, "{result:#?}");
        let result = client.rating_add_movie(550, 9.5).await.unwrap();
        assert!(result.success, "{result:#?}");
    }

    #[tokio::test]
    async fn tv() {
        // TV Show ID 1399 => Game of Thrones
        let client = client();
        let result = client.rating_delete_tv(1399).await.unwrap();
        assert!(result.success, "{result:#?}");
        let result = client.rating_add_tv(1399, 6.5).await.unwrap();
        assert!(result.success, "{result:#?}");
    }

    #[tokio::test]
    async fn episode() {
        // TV Show ID 1399 => Game of Thrones
        let client = client();
        let result = client.rating_delete_episode(1399, 8, 6).await.unwrap();
        assert!(result.success, "{result:#?}");
        let result = client.rating_add_episode(1399, 8, 6, 2.5).await.unwrap();
        assert!(result.success, "{result:#?}");
    }
}
