use serde::Deserialize;

pub(crate) mod product;

pub(crate) mod product_variant;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ProductsConnection {
    First(u32),
    Last(u32),
}
