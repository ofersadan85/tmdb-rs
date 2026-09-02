#![allow(dead_code)]

/// Public TMDB domain models organized by API chapter.
///
/// The TMDB reference groups the API by chapter (auth, movies, people, reviews,
/// collections, and so on). This module mirrors that organization so the library
/// stays aligned with the upstream API documentation and can grow without a
/// single monolithic model file.
pub mod account;
pub mod auth;
pub mod certifications;
pub mod changes;
pub mod collections;
pub mod common;
pub mod companies;
pub mod discover;
pub mod episodes;
pub mod favorites;
pub mod genres;
pub mod images;
pub mod keywords;
pub mod lists;
pub mod movies;
pub mod people;
pub mod reviews;
pub mod search;
pub mod session;
pub mod translations;
pub mod trending;
pub mod tv;
pub mod watchlist;
