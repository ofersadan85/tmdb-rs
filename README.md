# The Movie Database

![The Movie Database](https://www.themoviedb.org/assets/2/v4/logos/408x161-powered-by-rectangle-green-bb4301c10ddc749b4e79463811a68afebeae66ef43d17bcfd8ff0e60ded7ce99.png)

This is a wrapper around the [TMDb API](https://developers.themoviedb.org/3).

## Compatibility

This crate is updated to the latest API and includes new features, bug fixes, and improvements, including:

* Support for both V3 and V4 of the TMDb API
* Support for async requests

## Usage

```rust
use tmdb::model::*;
use tmdb::themoviedb::*;

fn main() {
    let tmdb = TMDb { api_key: env!("TMDB_API_KEY"), language: "en" };

    let movies = tmdb.search()
        .title("Interstellar")
        .year(2014)
        .execute()
        .unwrap();

    let id = movies.results[0].id;

    let interstellar: Movie = tmdb.fetch()
        .id(id)
        .execute()
        .unwrap();

    println!("{:#?}", interstellar);
}
```

## Actions

Currently there are 3 actions available:

* Searching
* Fetching
* Finding

### Searching

You can search for movies by `title` and `year`.

```rust
let page = tmdb.search()
    .title("Bicentennial Man")
    .year(1999)
    .execute()
    .unwrap();

let movies = page.results;
```

### Fetching

You can fetch a movie, when you know its ID. Then you get all the movie details.

```rust
let movie = tmdb.fetch()
    .id(157336)
    .execute()
    .unwrap();
```

When you don't have any movie ID, you can search for a movie and then easily fetch the full details.

```rust
let page = tmdb.search()
   .title("Bicentennial Man")
   .year(1999)
   .execute()
   .unwrap();

let movies = page.results;
let movie = movies[0].fetch(&tmdb).unwrap();
```

Furthermore you can request some more data with the [append to response](https://developers.themoviedb.org/3/getting-started/append-to-response) feature.

```rust
let movie = tmdb.fetch()
    .id(2277)
    .append_videos()
    .append_credits()
    .execute()
    .unwrap();
```

### Finding

[Finding](https://developers.themoviedb.org/3/find/find-by-id) a movie with an external ID is currently supported with IMDB IDs.

```rust
let find_result = tmdb.find()
    .imdb_id("tt0816692")
    .execute()
    .unwrap();

let movies = find_result.movie_results;
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
  * [ ] Details
  * [ ] Images
  * [ ] Translations
* [ ] Companies
  * [ ] Details
  * [ ] Alternative Names
  * [ ] Images
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
* [ ] Networks
  * [ ] Details
  * [ ] Alternative Names
  * [ ] Images
* [ ] People Lists
  * [ ] Popular
* [ ] People
  * [ ] Details
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
* [ ] Trending
  * [ ] All
  * [ ] Movies
  * [ ] People
  * [ ] TV
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

## Acknowledgements

* [The Movie Database (TMDb)](https://www.themoviedb.org/)
