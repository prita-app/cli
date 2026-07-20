//! Client library for the Prita API.
//!
//! Talks to the GraphQL endpoint (`POST /graphql`) with an application token
//! (`Authorization: Bearer prita_...`). Output formatting lives in the CLI crate,
//! not here.
//!
//! No code is shared with the server. Typed queries are checked at compile time
//! against the vendored `schema.graphql`, a copy of the API's SDL.

pub mod config;
pub mod error;
pub mod graphql;

mod client;

pub use client::PritaClient;
pub use error::Error;
