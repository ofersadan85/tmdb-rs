# The Movie Database

![The Movie Database](https://www.themoviedb.org/assets/2/v4/logos/408x161-powered-by-rectangle-green-bb4301c10ddc749b4e79463811a68afebeae66ef43d17bcfd8ff0e60ded7ce99.png)

This is a wrapper around the [TMDb API](https://developers.themoviedb.org/3).

## Compatibility

This crate is updated to the latest API and includes new features, bug fixes, and improvements, including:

* Support for both V3 and V4 of the TMDb API
* Support for async requests

## Usage

The library is centered around `TmdbClient`, which is created from a bearer token and then used to call TMDB endpoints.

```rust,no_run
use tmdb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TmdbClient::from_token("dummy-token")?;

    let movies = client.search_simple::<Movie>("Inception").await?;
    println!("First result: {}", movies.results[0].title);

    let account = client.account_details().await?;
    println!("Signed in as {}", account.username);

    let favorites = client
        .account_favorites::<Movie>(&account, AccountQuery::default())
        .await?;
    println!("Favorite movies: {}", favorites.results.len());

    let by_external_id = client
        .find_by_external_id("tt0816692", ExternalSourceId::Imdb, None)
        .await?;
    println!("External ID lookup succeeded: {:?}", by_external_id);

    Ok(())
}
```

### Searching

```rust,no_run
use tmdb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TmdbClient::from_token("dummy-token")?;
    let results = client.search_simple::<Movie>("Interstellar").await?;
    println!("{} results found", results.total_results);
    Ok(())
}
```

### Account details and lists

```rust,no_run
use tmdb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TmdbClient::from_token("dummy-token")?;
    let account = client.account_details().await?;
    let favorites = client
        .account_favorites::<Movie>(&account, AccountQuery::default())
        .await?;
    let watchlist = client
        .account_watchlist::<Movie>(&account, AccountQuery::default())
        .await?;

    println!("favorites={}, watchlist={}", favorites.results.len(), watchlist.results.len());
    Ok(())
}
```

### Adding and removing ratings

```rust,no_run
use tmdb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TmdbClient::from_token("dummy-token")?;
    client.rating_add_movie(550, 8.5).await?;
    client.rating_delete_movie(550).await?;
    Ok(())
}
```

These methods are available on `TmdbClient` for movies, TV shows, and episodes:

```rust,no_run
use tmdb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TmdbClient::from_token("dummy-token")?;
    client.rating_add_tv(1399, 9.0).await?;
    client.rating_add_episode(1399, 1, 1, 8.5).await?;
    Ok(())
}
```

### Finding by external ID

```rust,no_run
use tmdb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TmdbClient::from_token("dummy-token")?;
    let response = client
        .find_by_external_id("tt0816692", ExternalSourceId::Imdb, None)
        .await?;
    println!("Find response: {:?}", response);
    Ok(())
}
```
