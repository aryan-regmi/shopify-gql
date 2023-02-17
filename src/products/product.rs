#![allow(unused)]

use serde::Deserialize;

use crate::{
    common::{Id, InventoryItem, Money, WeightUnit},
    utils::{
        run_query, QueryResponse, ResponseTypes, ShopifyConfig, ShopifyGqlError, ShopifyResult,
    },
};
use std::collections::HashMap;

use super::ProductVariant;

/// The possible product statuses.
#[derive(Debug, Deserialize)]
enum ProductStatus {
    /// The product is ready to sell and can be published to sales channels and apps.
    /// Products with an active status aren't automatically published to sales channels,
    /// such as the online store, or apps. By default, existing products are set to active.
    ACTIVE,

    /// The product is no longer being sold and isn't available to customers on sales
    /// channels and apps.
    ARCHIVED,

    /// The product isn't ready to sell and is unavailable to customers on sales
    /// channels and apps. By default, duplicated and unarchived products are set to draft.
    DRAFT,
}

/// The Product resource lets you manage products in a merchant’s store
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Product {
    /// A globally-unique identifier.
    id: Id,

    /// The product type specified by the merchant.
    product_type: String,

    /// The product status. This controls visibility across all channels.
    status: ProductStatus,

    /// A comma separated list of tags associated with the product
    tags: Vec<String>,

    /// The title of the product.
    title: String,

    /// The quantity of inventory in stock.
    total_inventory: i32,

    /// The number of variants that are associated with the product.
    total_variants: i32,

    /// The name of the product's vendor.
    vendor: String,
    // /// A list of variants associated with the product.
    // variants: Vec<ProductVariant>,
}

impl Product {
    fn from_query(id: Id) -> ProductQueryBuilder {
        ProductQueryBuilder {
            id,
            fields: HashMap::new(),
        }
    }
}

struct ProductQueryBuilder {
    id: Id,
    fields: HashMap<String, String>,
}

// TODO: Add actual values in the HashMaps
impl ProductQueryBuilder {
    fn product_type(mut self) -> Self {
        let key = "productType".into();
        let value = "productType".into();
        self.fields.insert(key, value);
        self
    }

    fn status(mut self) -> Self {
        let key = "status".into();
        let value = "status".into();
        self.fields.insert(key, value);
        self
    }

    fn tags(mut self) -> Self {
        let key = "tags".into();
        let value = "tags".into();
        self.fields.insert(key, value);
        self
    }

    fn title(mut self) -> Self {
        let key = "title".into();
        let value = "title".into();
        self.fields.insert(key, value);
        self
    }

    fn total_inventory(mut self) -> Self {
        let key = "totalInventory".into();
        let value = "totalInventory".into();
        self.fields.insert(key, value);
        self
    }

    fn total_variants(mut self) -> Self {
        let key = "totalVariants".into();
        let value = "totalVariants".into();
        self.fields.insert(key, value);
        self
    }

    fn vendor(mut self) -> Self {
        let key = "vendor".into();
        let value = "vendor".into();
        self.fields.insert(key, value);
        self
    }

    async fn build(self, config: ShopifyConfig) -> ShopifyResult<Product> {
        let fields = self.fields.into_values().map(|k| k).collect::<Vec<_>>();
        let fields = fields.join(",\n");

        // Build query
        let query = format!(
            "query {{ product(id: \"{}\") {{ {} }} }}",
            self.id.as_str(),
            fields
        );

        // Run query
        let prod_res = run_query(config, query).await?;
        match prod_res {
            QueryResponse::Data { data } => match data {
                ResponseTypes::Product(p) => {
                    dbg!(&p);
                    Ok(p.to_product())
                    // Ok(p)
                }
            },
            QueryResponse::Errors(e) => Err(ShopifyGqlError::ResponseError(format!(
                "Invalid Product was returned: {}",
                e.to_string()
            ))),
        }
    }
}

/// The Product resource lets you manage products in a merchant’s store
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", rename = "product")]
pub(crate) struct ProductResponse {
    /// A globally-unique identifier.
    id: Id,

    /// The product type specified by the merchant.
    product_type: Option<String>,

    /// The product status. This controls visibility across all channels.
    status: ProductStatus,

    /// A comma separated list of tags associated with the product
    tags: Vec<String>,

    /// The title of the product.
    title: Option<String>,

    /// The quantity of inventory in stock.
    total_inventory: Option<String>,

    /// The number of variants that are associated with the product.
    total_variants: Option<String>,

    /// The name of the product's vendor.
    vendor: Option<String>,
    // /// A list of variants associated with the product.
    // variants: Vec<ProductVariant>,
}

impl ProductResponse {
    fn to_product(self) -> Product {
        let id = self.id;
        let product_type = match self.product_type {
            Some(p) => p,
            None => "".into(),
        };
        let status = self.status;
        let tags = self.tags;
        let title = match self.title {
            Some(t) => t,
            None => "".into(),
        };
        let total_inventory = match self.total_inventory {
            Some(i) => i.parse().unwrap(),
            None => 0,
        };
        let total_variants = match self.total_variants {
            Some(v) => v.parse().unwrap(),
            None => 0,
        };
        let vendor = match self.vendor {
            Some(v) => v,
            None => "".into(),
        };

        Product {
            id,
            product_type,
            status,
            tags,
            title,
            total_inventory,
            total_variants,
            vendor,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ShopifyGqlError;

    use super::*;

    #[tokio::test]
    async fn tst() -> Result<(), ShopifyGqlError> {
        let prod = Product::from_query(Id::product("7395184804017"))
            // .product_type()
            // .vendor()
            // .total_variants()
            // .total_inventory()
            // .tags()
            // .status()
            .title()
            .build(ShopifyConfig::from_env()?)
            .await?;

        dbg!(prod);

        Ok(())
    }
}
