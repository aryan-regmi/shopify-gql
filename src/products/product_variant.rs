#![allow(unused)]

use crate::{
    common::{Id, Money, WeightUnit},
    utils::{run_query, ResponseTypes, ShopifyConfig, ShopifyConnection, ShopifyResult},
};
use serde::Deserialize;

use super::{
    product::{Product, ProductQueryBuilder},
    ProductsConnection,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductVariant {
    id: Id,
    compare_at_price: Option<Money>,
    inventory_quantity: Option<i32>,
    price: Option<Money>,
    product: Option<Product>,
    sku: Option<String>,
    title: Option<String>,
    weight: Option<f64>,
    weight_unit: Option<WeightUnit>,
}

impl ProductVariant {
    pub(crate) fn id(&self) -> &Id {
        &self.id
    }

    pub(crate) fn compare_at_price(&self) -> Option<&Money> {
        self.compare_at_price.as_ref()
    }

    pub(crate) fn inventory_quantity(&self) -> Option<i32> {
        self.inventory_quantity
    }

    pub(crate) fn price(&self) -> Option<&Money> {
        self.price.as_ref()
    }

    pub(crate) fn product(&self) -> Option<&Product> {
        self.product.as_ref()
    }

    pub(crate) fn sku(&self) -> Option<&String> {
        self.sku.as_ref()
    }

    pub(crate) fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub(crate) fn weight(&self) -> Option<f64> {
        self.weight
    }

    pub(crate) fn weight_unit(&self) -> Option<&WeightUnit> {
        self.weight_unit.as_ref()
    }
}

/// All possible queries and mutations on a `Product`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ProductVariantQueryType {
    ProductVariant,
    ProductVariants(ProductsConnection),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductVariantQueryBuilder {
    id: Id,
    fields: Vec<String>,
    query_type: ProductVariantQueryType,
}

impl ProductVariantQueryBuilder {
    pub(crate) fn product_variant(id: Id) -> Self {
        let mut fields = Vec::new();
        fields.push("id".into());

        let query_type = ProductVariantQueryType::ProductVariant;

        ProductVariantQueryBuilder {
            id,
            fields,
            query_type,
        }
    }

    pub(crate) fn product_variants(conn: ProductsConnection) -> Self {
        let mut fields = Vec::new();
        fields.push("id".into());

        let query_type = ProductVariantQueryType::ProductVariants(conn);
        let id = Id::default();

        ProductVariantQueryBuilder {
            id,
            fields,
            query_type,
        }
    }

    pub(crate) fn compare_at_price(mut self) -> Self {
        self.fields.push("compareAtPrice".into());
        self
    }

    pub(crate) fn inventory_quantity(mut self) -> Self {
        self.fields.push("inventoryQuantity".into());
        self
    }

    pub(crate) fn price(mut self) -> Self {
        self.fields.push("price".into());
        self
    }

    /// NOTE: Calling `.variants()` on the `product_query` will cause an infinte cycle.
    pub(crate) fn product(mut self, product_query: ProductQueryBuilder) -> Self {
        let prod_str = format!("product {{ {} }}", product_query.fields().join("\n,"));

        self.fields.push(prod_str);
        self
    }

    pub(crate) fn sku(mut self) -> Self {
        self.fields.push("sku".into());
        self
    }

    pub(crate) fn title(mut self) -> Self {
        self.fields.push("title".into());
        self
    }

    pub(crate) fn weight(mut self) -> Self {
        self.fields.push("weight".into());
        self
    }

    pub(crate) fn weight_unit(mut self) -> Self {
        self.fields.push("weightUnit".into());
        self
    }

    pub(crate) fn fields(&self) -> &[String] {
        self.fields.as_ref()
    }

    pub(crate) fn query_type(&self) -> &ProductVariantQueryType {
        &self.query_type
    }

    pub(crate) async fn build(self, config: ShopifyConfig) -> ShopifyResult<ProductVariant> {
        let fields = self.fields.join("\n,");

        let query = match self.query_type {
            ProductVariantQueryType::ProductVariant => {
                format!(
                    "query {{ productVariant(id: \"{}\") {{ {} }} }}",
                    self.id.inner(),
                    fields
                )
            }

            ProductVariantQueryType::ProductVariants(conn_type) => match conn_type {
                ProductsConnection::First(n) => {
                    format!(
                        "query {{ productVariant(first: {}) {{ edges {{ node {{ {} }} }} }}  }}",
                        n, fields
                    )
                }

                ProductsConnection::Last(n) => {
                    format!(
                        "query {{ productVariant(last: {}) {{ edges {{ node {{ {} }} }} }}  }}",
                        n, fields
                    )
                }
            },
        };

        let res = run_query(config, query).await?;
        match res.data {
            ResponseTypes::ProductVariant(v) => Ok(v),

            _ => unreachable!(), // FIX: Replace this with an Error
        }
    }
}
