/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
//! Official Rust client for [OpenSearch](https://opensearch.org/)
//!
//! `OpenSearch` is an official Rust client for OpenSearch, providing an efficient asynchronous
//! client for all stable OpenSearch APIs that's easy to use.
//!
//! # Versions and Compatibility
//!
//! | Rust client | OpenSearch    |
//! |-------------|---------------|
//! | 1.x         | 1.x           |
//! | 2.x         | 2.x, 1.x^     |
//! - ^: With the exception of some previously deprecated APIs 
//!
//! A major version of the client is compatible with the same major version of OpenSearch.
//! Since OpenSearch is developed following [Semantic Versioning](https://semver.org/) principles,
//! Any minor/patch version of the client can be used against any minor/patch version of OpenSearch
//! **within the same major version lineage**. For example,
//!
//! - A `1.5.0` client can be used against `1.0.0` OpenSearch
//! - A `1.4.0` client can be used against `1.5.1` OpenSearch
//!
//! In the former case, a 1.5.0 client may contain additional API functions that are not available
//! in 1.0.0 OpenSearch. In this case, these APIs cannot be used, but for any APIs available in
//! OpenSearch, the respective API functions on the client will be compatible.
//!
//! In the latter case, a 1.4.0 client won't contain API functions for APIs that are introduced in
//! OpenSearch 1.5.0+, but for all other APIs available in OpenSearch, the respective API
//! functions on the client will be compatible.
//! 
//! In some instances, a new major version of OpenSearch may remain compatible with an
//! older major version of the client, which may not warrant a need to update the client.
//! Please consult COMPATIBILITY.md for more details.
//!
//! **No compatibility assurances are given between different major versions of the client and
//! OpenSearch**. Major differences likely exist between major versions of OpenSearch, particularly
//! around request and response object formats, but also around API urls and behaviour.
//!
//! # Features
//!
//! The following are a list of Cargo features that can be enabled or disabled:
//!
//! - **native-tls** *(enabled by default)*: Enables TLS functionality provided by `native-tls`.
//! - **rustls-tls**: Enables TLS functionality provided by `rustls`.
//! - **beta-apis**: Enables beta APIs. Beta APIs are on track to become stable and permanent features. Use them with
//!    caution because it is possible that breaking changes are made to these APIs in a minor version.
//! - **experimental-apis**: Enables experimental APIs. Experimental APIs are just that - an experiment. An experimental
//!   API might have breaking changes in any future version, or it might even be removed entirely. This feature also
//!   enables `beta-apis`.
//! - **aws-auth**: Enables authentication with Amazon OpenSearch and OpenSearch Serverless.
//!   Performs AWS SigV4 signing using credential types from `aws-types`.
//!
//! # Getting started
//!
//! Add the `opensearch` crate and version to Cargo.toml. Choose the version that is compatible with
//! the version of OpenSearch you're using
//!
//! ```toml,no_run
//! [dependencies]
//! opensearch = "1.0.0"
//! ```
//! The following _optional_ dependencies may also be useful to create requests and read responses
//!
//! ```toml,no_run
//! serde = "~1"
//! serde_json = "~1"
//! ```
//!
//! ### Async support with tokio
//!
//! The client uses [`reqwest`](https://crates.io/crates/reqwest) to make HTTP calls, which internally uses
//! the [`tokio`](https://crates.io/crates/tokio) runtime for async support. As such, you may require
//! to take a dependency on `tokio` in order to use the client. For example, in Cargo.toml, you may
//! need the following dependency
//!
//! ```toml,no_run
//! tokio = { version = "*", features = ["full"] }
//! ```
//!
//! and to attribute async main function with `#[tokio::main]`
//!
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // your code ...
//!     Ok(())
//! }
//! ```
//!
//! and attribute test functions with `#[tokio::test]`
//!
//! ```rust,no_run
//! #[tokio::test]
//! async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
//!     // your code ...
//!     Ok(())
//! }
//! ```
//!
//! ## Create a client
//!
//! To create a client to make API calls to OpenSearch running on `http://localhost:9200`
//!
//! ```rust,no_run
//! # use opensearch::OpenSearch;
//! let client = OpenSearch::default();
//! ```
//!
//! Alternatively, you can create a client to make API calls against OpenSearch running on a
//! specific [url::Url]
//!
//! ```rust,no_run
//! # use opensearch::{
//! #     Error, OpenSearch,
//! #     http::transport::{Transport, SingleNodeConnectionPool}
//! # };
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let transport = Transport::single_node("https://example.com")?;
//! let client = OpenSearch::new(transport);
//! # Ok(())
//! # }
//! ```
//!
//! More control over how a [Transport](http::transport::Transport) is built can be
//! achieved using [TransportBuilder](http::transport::TransportBuilder) to build a transport, and
//! passing it to [OpenSearch::new] create a new instance of [OpenSearch]
//!
//! ```rust,no_run
//! # use opensearch::{
//! #     Error, OpenSearch,
//! #     http::transport::{TransportBuilder,SingleNodeConnectionPool},
//! # };
//! # use url::Url;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let url = Url::parse("https://example.com")?;
//! let conn_pool = SingleNodeConnectionPool::new(url);
//! let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
//! let client = OpenSearch::new(transport);
//! # Ok(())
//! # }
//! ```
//!
//! ## Making API calls
//!
//! The client exposes all stable OpenSearch APIs, either on the root [OpenSearch] client,
//! or on a _namespace_ client that groups related APIs, such as [Cat](cat::Cat), which groups the
//! Cat related APIs. All API functions are `async` and can be `await`ed.
//!
//! The following makes an API call to the cat indices API
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, Error, cat::CatIndicesParts};
//! # use url::Url;
//! # use serde_json::{json, Value};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let response = client
//!     .cat()
//!     .indices(CatIndicesParts::Index(&["*"]))
//!     .send()
//!     .await?;
//!
//! let response_body = response.json::<Value>().await?;
//! for record in response_body.as_array().unwrap() {
//!     // print the name of each index
//!     println!("{}", record["index"].as_str().unwrap());
//! }
//! # Ok(())
//! # }
//! ```
//! For APIs that contain parts of the Url path to be provided by the consumer, the Url path
//! variants are modelled as an `enum`, such as [CatIndicesParts](cat::CatIndicesParts) in the above example, which models
//! the variants of the [CatIndices](cat::CatIndices) API.
//!
//! ### Indexing
//!
//! Indexing a single document can be achieved with the index API
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, Error, SearchParts, IndexParts};
//! # use url::Url;
//! # use serde_json::{json, Value};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let response = client
//!     .index(IndexParts::IndexId("tweets", "1"))
//!     .body(json!({
//!         "id": 1,
//!         "user": "kimchy",
//!         "post_date": "2009-11-15T00:00:00Z",
//!         "message": "Trying out OpenSearch, so far so good?"
//!     }))
//!     .send()
//!     .await?;
//!
//! let successful = response.status_code().is_success();
//! # Ok(())
//! # }
//! ```
//!
//! For indexing multiple documents, the bulk API is a better option, allowing multiple operations
//! to be sent in one API call
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, Error, IndexParts, BulkParts, http::request::JsonBody};
//! # use url::Url;
//! # use serde_json::{json, Value};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);
//!
//! // add the first operation and document
//! body.push(json!({"index": {"_id": "1"}}).into());
//! body.push(json!({
//!     "id": 1,
//!     "user": "kimchy",
//!     "post_date": "2009-11-15T00:00:00Z",
//!     "message": "Trying out OpenSearch, so far so good?"
//! }).into());
//!
//! // add the second operation and document
//! body.push(json!({"index": {"_id": "2"}}).into());
//! body.push(json!({
//!     "id": 2,
//!     "user": "forloop",
//!     "post_date": "2020-01-08T00:00:00Z",
//!     "message": "Bulk indexing with the rust client, yeah!"
//! }).into());
//!
//! let response = client
//!     .bulk(BulkParts::Index("tweets"))
//!     .body(body)
//!     .send()
//!     .await?;
//!
//! let response_body = response.json::<Value>().await?;
//! let successful = response_body["errors"].as_bool().unwrap() == false;
//! # Ok(())
//! # }
//! ```
//! ### Searching
//!
//! The following makes an API call to `tweets/_search` with the json body
//! `{"query":{"match":{"message":"OpenSearch"}}}`
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, Error, SearchParts};
//! # use url::Url;
//! # use serde_json::{json, Value};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let response = client
//!     .search(SearchParts::Index(&["tweets"]))
//!     .from(0)
//!     .size(10)
//!     .body(json!({
//!         "query": {
//!             "match": {
//!                 "message": "OpenSearch rust"
//!             }
//!         }
//!     }))
//!     .send()
//!     .await?;
//!
//! let response_body = response.json::<Value>().await?;
//! let took = response_body["took"].as_i64().unwrap();
//! for hit in response_body["hits"]["hits"].as_array().unwrap() {
//!     // print the source document
//!     println!("{:?}", hit["_source"]);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Request bodies
//!
//! For APIs that expect JSON, the `body` associated function of the API constrains the input
//! to a type that implements [serde::Serialize] trait. An example of this was the indexing a single
//! document example above.
//!
//! Some APIs expect newline delimited JSON
//! (NDJSON) however, so the `body` associated for these APIs constrain the input to a vector of
//! types that implement [Body](http::request::Body) trait.  An example of this was the bulk indexing multiple documents
//! above.
//!
//! The [Body](http::request::Body) trait represents the body of an API call, allowing for different body implementations.
//! As well as those to represent JSON and NDJSON, a few other types also have implementations for
//! [Body](http::request::Body), such as byte slice. Whilst these can't be passed to the API functions directly,
//! [OpenSearch::send] can be used
//!
//! ```rust,no_run
//! # use opensearch::{http::{Method,headers::HeaderMap}, OpenSearch, Error, SearchParts};
//! # use url::Url;
//! # use serde_json::{json, Value};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let body = b"{\"query\":{\"match_all\":{}}}";
//!
//! let response = client
//!     .send(Method::Post,
//!         SearchParts::Index(&["tweets"]).url().as_ref(),
//!         HeaderMap::new(),
//!         Option::<&Value>::None,
//!         Some(body.as_ref()),
//!         None,
//!     )
//!     .await?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Amazon OpenSearch and OpenSearch Serverless
//!
//! For authenticating against an Amazon OpenSearch or OpenSearch Serverless endpoint using AWS SigV4 request signing,
//! you must enable the `aws-auth` feature, then pass the AWS credentials to the [TransportBuilder](http::transport::TransportBuilder).
//! The easiest way to retrieve AWS credentials in the required format is to use [aws-config](https://docs.rs/aws-config/latest/aws_config/).
//!
//! ```toml,no_run
//! [dependencies]
//! opensearch = { version = "1", features = ["aws-auth"] }
//! aws-config = "0.10"
//! ```
//!
//! ```rust,no_run
//! # use aws_config::meta::region::RegionProviderChain;
//! # use opensearch::{
//! #     Error, OpenSearch,
//! #     http::transport::{TransportBuilder,SingleNodeConnectionPool},
//! # };
//! # use url::Url;
//! # use std::convert::TryInto;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # #[cfg(feature = "aws-auth")] {
//! let creds = aws_config::load_from_env().await;
//! let url = Url::parse("https://...")?;
//! let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
//! let aws_config = aws_config::from_env().region(region_provider).load().await.clone();
//! let conn_pool = SingleNodeConnectionPool::new(url);
//! let transport = TransportBuilder::new(conn_pool)
//!     .auth(aws_config.clone().try_into()?)
//!     .service_name("es") // use "aoss" for OpenSearch Serverless
//!     .build()?;
//! let client = OpenSearch::new(transport);
//! # }
//! # Ok(())
//! # }
//! ```

#![doc(
    html_logo_url = "https://github.com/opensearch-project/opensearch-rs/raw/main/OpenSearch.svg"
)]
// TODO: turn on before releasing :) Will require adding documentation within all REST API specs
// #![deny(missing_docs)]

// also test examples in README
// source: https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790
#[cfg(doctest)]
mod readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../../README.md"));
}

pub mod auth;
pub mod cert;
pub mod http;
pub mod models;
pub mod params;

// GENERATED-BEGIN:namespace-modules
// Generated code - do not edit until the next GENERATED-END marker

pub mod cat;
pub mod cluster;
pub mod dangling_indices;
pub mod indices;
pub mod ingest;
pub mod nodes;
pub mod snapshot;
pub mod tasks;
pub mod text_structure;
// GENERATED-END

mod client;
mod error;
mod root;

// exposes types within modules at the library root level
pub use crate::{client::*, error::*, http::transport::DEFAULT_ADDRESS, root::*};
use serde::{
    de,
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData, str::FromStr};
use void::Void;

/// Deserializes a string or a map into a struct `T` that implements `FromStr`
pub(crate) fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

/// Deserializes a string or a sequence of strings into a Vec<String>
pub(crate) fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec;

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or seq of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_string()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec)
}

#[cfg(test)]
pub mod tests {
    use crate::SearchParts;

    #[test]
    fn build_search_on_all_indices() {
        let parts = SearchParts::None;
        let url = parts.url();
        assert_eq!(url, "/_search");
    }

    #[test]
    fn build_search_on_selected_indices() {
        let parts = SearchParts::Index(&["index-1", "index-2"]);
        let url = parts.url();
        assert_eq!(url, "/index-1,index-2/_search");
    }

    #[test]
    fn percent_encode_characters() {
        let parts = SearchParts::Index(&[" !\"#$%&'\\()*+,-./:;<=>?@[\\]^_`{|}~"]);
        let url = parts.url();
        assert_eq!(url, "/%20%21%22%23%24%25%26%27%5C%28%29*%2B,-.%2F%3A%3B%3C%3D%3E%3F%40%5B%5C%5D%5E_%60%7B%7C%7D%7E/_search");
    }
}
