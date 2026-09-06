# The Movie Database

![The Movie Database](https://www.themoviedb.org/assets/2/v4/logos/408x161-powered-by-rectangle-green-bb4301c10ddc749b4e79463811a68afebeae66ef43d17bcfd8ff0e60ded7ce99.png)

This is a wrapper around the [TMDb API](https://developers.themoviedb.org/3).

## Compatibility

This crate is updated to the latest API and includes new features, bug fixes, and improvements, including:

* Support for both V3 and V4 of the `TMDb` API
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

## API Coverage Checklist

* [x] Account
  * [x] Details
  * [x] Add Favorite
  * [x] Add to Watchlist
  * [x] Favorite Movies
  * [x] Favorite TV
  * [x] Lists
  * [x] Rated Movies
  * [x] Rated TV
  * [x] Rated TV Episodes
  * [x] Watchlist Movies
  * [x] Watchlist TV
* [ ] Authentication
  * [ ] Create Guest Session
  * [ ] Create Request Token
  * [ ] Create Session
  * [ ] Create Session from V4 Token
  * [ ] Create Session with Login
  * [ ] Delete Session
  * [ ] Validate Key
* [ ] Certifications
  * [ ] Movie Certifications
  * [ ] TV Certifications
* [ ] Changes
  * [ ] Movie List
  * [ ] People List
  * [ ] TV List
* [ ] Collections
  * [x] Details
  * [ ] Images
  * [ ] Translations
* [x] Companies
  * [x] Details
  * [x] Alternative Names
  * [x] Images
* [ ] Configuration
  * [ ] Details
  * [ ] Countries
  * [ ] Jobs
  * [ ] Languages
  * [ ] Primary Translations
  * [ ] Timezones
* [ ] Credits
  * [ ] Details
* [ ] Discover
  * [ ] Movie
  * [ ] TV
* [x] Find
  * [x] By ID
* [ ] Genres
  * [ ] Movie List
  * [ ] TV List
* [ ] Guest Sessions
  * [ ] Rated Movies
  * [ ] Rated TV
  * [ ] Rated TV Episodes
* [ ] Keywords
  * [ ] Details
  * [ ] Movies
* [ ] Lists
  * [ ] Add Movie
  * [ ] Check Item Status
  * [ ] Clear
  * [ ] Create
  * [ ] Delete
  * [ ] Remove Movie
* [ ] Movie Lists
  * [ ] Now Playing
  * [ ] Popular
  * [ ] Top Rated
  * [ ] Upcoming
* [ ] Movies
  * [ ] Details
  * [ ] Account States
  * [ ] Alternative Titles
  * [ ] Changes
  * [ ] Credits
  * [ ] External IDs
  * [ ] Images
  * [ ] Keywords
  * [ ] Latest
  * [ ] Lists
  * [ ] Recommendations
  * [ ] Release Dates
  * [ ] Reviews
  * [ ] Similar
  * [ ] Translations
  * [ ] Videos
  * [ ] Watch Providers
  * [x] Add Rating
  * [x] Delete Rating
* [x] Networks
  * [x] Details
  * [x] Alternative Names
  * [x] Images
* [ ] People Lists
  * [ ] Popular
* [ ] People
  * [x] Details
  * [ ] Changes
  * [ ] Combined Credits
  * [ ] External IDs
  * [ ] Images
  * [ ] Latest
  * [ ] Movie Credits
  * [ ] TV Credits
  * [ ] Tagged Images
  * [ ] Translations
* [ ] Reviews
  * [ ] Details
* [x] Search
  * [x] Collection
  * [x] Company
  * [x] Keyword
  * [x] Movie
  * [x] Multi
  * [x] Person
  * [x] TV
* [x] Trending
  * [x] All
  * [x] Movies
  * [x] People
  * [x] TV
* [ ] TV Series Lists
  * [ ] Airing Today
  * [ ] On The Air
  * [ ] Popular
  * [ ] Top Rated
* [ ] TV Series
  * [ ] Details
  * [ ] Account States
  * [ ] Aggregate Credits
  * [ ] Alternative Titles
  * [ ] Changes
  * [ ] Content Ratings
  * [ ] Credits
  * [ ] Episode Groups
  * [ ] External IDs
  * [ ] Images
  * [ ] Keywords
  * [ ] Latest
  * [ ] Lists
  * [ ] Recommendations
  * [ ] Reviews
  * [ ] Screened Theatrically
  * [ ] Similar
  * [ ] Translations
  * [ ] Videos
  * [ ] Watch Providers
  * [x] Add Rating
  * [x] Delete Rating
* [ ] TV Seasons
  * [ ] Details
  * [ ] Account States
  * [ ] Aggregate Credits
  * [ ] Changes
  * [ ] Credits
  * [ ] External IDs
  * [ ] Images
  * [ ] Translations
  * [ ] Videos
  * [ ] Watch Providers
* [ ] TV Episodes
  * [ ] Details
  * [ ] Account States
  * [ ] Changes
  * [ ] Credits
  * [ ] External IDs
  * [ ] Images
  * [ ] Translations
  * [ ] Videos
  * [x] Add Rating
  * [x] Delete Rating
* [ ] TV Episode Groups
  * [ ] Details
* [ ] Watch Providers
  * [ ] Available Regions
  * [ ] Movie Providers
  * [ ] TV Providers
