#![allow(unused)]

use crate::utils::{ShopifyGqlError, ShopifyResult};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Id(String);

impl Id {
    fn is_numeric(id: &str) -> bool {
        let mut ret = true;
        for c in id.chars() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => continue,

                _ => return false,
            }
        }

        ret
    }

    pub(crate) fn product(id: &str) -> ShopifyResult<Self> {
        // Validate input (must be numbers only)
        if !Id::is_numeric(id) {
            return Err(ShopifyGqlError::InvalidId(id.into()));
        }

        Ok(Self(format!("gid://shopify/Product/{}", id)))
    }

    pub(crate) fn product_variant(id: &str) -> ShopifyResult<Self> {
        // Validate input (must be numbers only)
        if !Id::is_numeric(id) {
            return Err(ShopifyGqlError::InvalidId(id.into()));
        }

        Ok(Self(format!("gid://shopify/ProductVariant/{}", id)))
    }

    pub(crate) fn inner(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", try_from = "String")]
pub(crate) struct Money(pub f64);

impl TryFrom<String> for Money {
    type Error = ShopifyGqlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let v: f64 = value
            .parse()
            .map_err(|_| ShopifyGqlError::FloatParseError(value))?;

        Ok(Money(v))
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Node<T> {
    node: T,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Edges<T> {
    edges: Vec<Node<T>>,
}

impl<T> Edges<T> {
    pub(crate) fn to_inner_vec(&self) -> Vec<&T> {
        self.edges.iter().map(|e| &e.node).collect()
    }

    pub(crate) fn get_node(&self, idx: usize) -> &T {
        &self.edges[idx].node
    }
}
