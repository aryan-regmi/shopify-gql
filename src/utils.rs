#![allow(unused)]

use dotenvy::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use std::env::{self, VarError};
use thiserror::Error;

use crate::{
    bulk_mutations::BulkOperation,
    products::{product::Product, product_variant::ProductVariant},
};

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

    #[error("Invalid ID ({0}): The ID must only be numbers")]
    InvalidId(String),

    #[error("Unable to parse {0} as float")]
    FloatParseError(String),
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
#[serde(rename_all = "camelCase")]
pub(crate) enum ResponseTypes {
    Product(Product),

    ProductVariant(ProductVariant),

    ProductUpdate {
        product: Product,
    },

    #[serde(rename_all = "camelCase")]
    ProductVariantUpdate {
        product_variant: ProductVariant,
    },

    #[serde(rename_all = "camelCase")]
    BulkOperationRunQuery {
        bulk_operation: BulkOperation,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryResponse {
    pub(crate) data: ResponseTypes,
}

// TODO: Handle Shopify errors
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
    // let t: serde_json::Value = res.json().await?;
    // dbg!(t);
    // todo!()

    let ret: Result<QueryResponse, _> = res.json().await;
    let ret = match ret {
        Ok(r) => Ok(r),
        Err(e) => Err(ShopifyGqlError::ResponseError(format!(
            "Unable to parse response: {}",
            e
        ))),
    };

    ret
}
