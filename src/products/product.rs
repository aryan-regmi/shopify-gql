#![allow(unused)]

use crate::{
    common::{Id, InventoryItem, Money, WeightUnit},
    utils::ShopifyConfig,
};
use std::collections::HashMap;

use super::ProductVariant;

/// The possible product statuses.
#[derive(Debug)]
enum ProductStatus {
    /// The product is ready to sell and can be published to sales channels and apps.
    /// Products with an active status aren't automatically published to sales channels,
    /// such as the online store, or apps. By default, existing products are set to active.
    Active,

    /// The product is no longer being sold and isn't available to customers on sales
    /// channels and apps.
    Archived,

    /// The product isn't ready to sell and is unavailable to customers on sales
    /// channels and apps. By default, duplicated and unarchived products are set to draft.
    Draft,
}

/// The Product resource lets you manage products in a merchant’s store
#[derive(Debug)]
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

    /// A list of variants associated with the product.
    variants: Vec<ProductVariant>,
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

    fn build(self, config: ShopifyConfig) -> Product {
        let fields = self.fields.into_values().map(|k| k).collect::<Vec<_>>();
        let fields = fields.join(",\n");

        // Build query
        let query = format!(
            "query {{ product(id: \"{}\") {{ {} }} }}",
            self.id.as_str(),
            fields
        );

        // dbg!(query);
        eprintln!("{query}");

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst() {
        let prod = Product::from_query(Id::product("125"))
            .product_type()
            .vendor()
            .total_variants()
            .total_inventory()
            .tags()
            .status()
            .title()
            .build(ShopifyConfig::from_env());
    }
}
