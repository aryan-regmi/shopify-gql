use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ShopifyGqlError {}

type ShopifyResult<T> = Result<T, ShopifyGqlError>;

pub(crate) struct ShopifyConfig {
    /// The API endpoint.
    pub(crate) api_url: String,

    /// The API token used for authentication.
    pub(crate) api_token: String,

    /// TODO: The actual connection to the API endpoint.
    pub(crate) connection: Option<String>,
}

impl ShopifyConfig {
    pub(crate) fn init(api_url: String, api_token: String) -> Self {
        // TODO: Add actual reqwest connection
        let connection = Some("".into());

        Self {
            api_url,
            api_token,
            connection,
        }
    }

    pub(crate) fn from_env() -> Self {
        todo!()
    }
}

pub(crate) fn run_query(config: &ShopifyConfig, query: &str) -> ShopifyResult<()> {
    Ok(())
}
