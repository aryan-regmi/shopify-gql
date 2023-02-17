#![allow(unused)]

use dotenvy::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use std::env::{self, VarError};
use thiserror::Error;

use crate::products::{Product, ProductResponse, ProductVariant};

#[derive(Debug, Error)]
pub(crate) enum ShopifyGqlError {
    #[error("Environment varible not found")]
    EnvironmentVariableNotFound(#[from] VarError),

    #[error("The API token is invalid")]
    InvalidApiToken(#[from] InvalidHeaderValue),

    #[error("Unable to make POST request: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Unable to parse response: {0}")]
    ResponseError(String),
}

pub(crate) type ShopifyResult<T> = Result<T, ShopifyGqlError>;

#[derive(Debug)]
pub(crate) struct ShopifyConnection {
    client: Client,
    headers: HeaderMap,
}

#[derive(Debug)]
pub(crate) struct ShopifyConfig {
    /// The API endpoint.
    api_url: String,

    /// The API token used for authentication.
    api_token: String,

    /// TODO: The actual connection to the API endpoint.
    connection: Option<ShopifyConnection>,
}

impl ShopifyConfig {
    pub(crate) fn init(api_url: &str, api_token: &str) -> Self {
        // Construct necessary headers to pass to URL
        let headers = {
            let mut headers = HeaderMap::new();

            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/graphql"),
            );

            headers.insert(
                HeaderName::from_static("x-shopify-access-token"),
                HeaderValue::from_str(api_token).expect(
                    "[HeaderValue Error] The `x-shopify-access-token` header is an invalid string.",
                ),
            );

            headers
        };
        let client = reqwest::Client::new();
        let connection = Some(ShopifyConnection { client, headers });

        Self {
            connection,
            api_url: api_url.into(),
            api_token: api_url.into(),
        }
    }

    pub(crate) fn from_env() -> ShopifyResult<Self> {
        dotenv().ok(); // Load vars from .env file

        let api_url = env::var("API_URL")?;
        let api_token = env::var("API_TOKEN")?;

        Ok(Self::init(&api_url, &api_token))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", rename = "data")]
/// The various types of responses a query can result in.
pub(crate) enum ResponseTypes {
    /// Response returning a Shopify `Product`.
    // Product(Product),
    Product(ProductResponse),
    // /// Response returning a Shopify `ProductVariant`.
    // ProductVariant(ProductVariant),
    // /// Response returning a list of Shopify `Product`s.
    // Products(NodeList<Product>),
    //
    // /// Response returning a list of Shopify `ProductVariant`s.
    // Variants(NodeList<ProductVariant>),
    //
    // /// Response for a `BulkOperationRunQuery`.
    // BulkOperationRunQuery(BulkOperationRunQuery),
    //
    // /// Response for a `CurrentBulkOperation` query.
    // /// Holds info about the currently running bulk query.
    // CurrentBulkOperation(BulkOperation),
    //
    // /// Response for a `ProductVariantUpdate` query, which returns a [Product].
    // ProductVariantUpdate(Box<ResponseTypes>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
/// The most basic/top-level type in the query response.
pub(crate) enum QueryResponse {
    /// Represents all non-error responses.
    Data { data: ResponseTypes },

    /// Represents any errors.
    Errors(serde_json::Value),
}

pub(crate) async fn run_query(
    config: ShopifyConfig,
    query: String,
) -> ShopifyResult<QueryResponse> {
    let conn = match config.connection {
        Some(conn) => conn,
        None => unreachable!(),
    };
    let client = conn.client;
    let headers = conn.headers;
    let url = config.api_url;

    let res = client.post(url).headers(headers).body(query).send().await?;
    let ret: Result<QueryResponse, _> = res.json().await;
    let ret = match ret {
        Ok(r) => {
            dbg!(&r);
            Ok(r)
        }
        Err(e) => Err(ShopifyGqlError::ResponseError(format!(
            "Unable to parse response: {}",
            e
        ))),
    };

    ret
}
