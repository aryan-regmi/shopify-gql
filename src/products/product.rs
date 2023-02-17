#![allow(unused)]

use crate::{
    common::{Edges, Id, Node},
    utils::{run_query, ResponseTypes, ShopifyConfig, ShopifyResult},
};
use serde::Deserialize;

use super::{
    product_variant::{ProductVariant, ProductVariantQueryBuilder, ProductVariantQueryType},
    ProductsConnection,
};

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum ProductStatus {
    /// The product is ready to sell and can be published to sales channels and apps.
    ACTIVE,

    /// The product is no longer being sold and isn't available to customers on sales channels and apps.
    ARCHIVED,

    /// The product isn't ready to sell and is unavailable to customers on sales channels and apps.
    DRAFT,
}

// NOTE: Need to update `ProductQueryBuilder` anytime a field is added/changed.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Product {
    id: Id,

    title: Option<String>,

    status: Option<ProductStatus>,

    vendor: Option<String>,

    variants: Option<Edges<ProductVariant>>,
}

impl Product {
    pub(crate) fn get_first(num_products: u32) {}

    pub(crate) fn id(&self) -> &Id {
        &self.id
    }

    pub(crate) fn status(&self) -> Option<&ProductStatus> {
        self.status.as_ref()
    }

    pub(crate) fn vendor(&self) -> Option<&String> {
        self.vendor.as_ref()
    }

    pub(crate) fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub(crate) fn variants(&self) -> Option<&Edges<ProductVariant>> {
        self.variants.as_ref()
    }
}

/// All possible queries and mutations on a `Product`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ProductQueryType {
    Product,
}

// NOTE: This needs to be updated anytime a new field is added to `Product`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductQueryBuilder {
    id: Id,
    fields: Vec<String>,
    query_type: ProductQueryType,
}

impl ProductQueryBuilder {
    pub(crate) fn product(id: Id) -> Self {
        let mut fields = Vec::new();
        fields.push("id".into());

        let query_type = ProductQueryType::Product;

        ProductQueryBuilder {
            id,
            fields,
            query_type,
        }
    }

    pub(crate) fn status(mut self) -> Self {
        self.fields.push("status".into());
        self
    }

    pub(crate) fn vendor(mut self) -> Self {
        self.fields.push("vendor".into());
        self
    }

    pub(crate) fn title(mut self) -> Self {
        self.fields.push("title".into());
        self
    }

    pub(crate) fn variants(mut self, variants_query: ProductVariantQueryBuilder) -> Self {
        // Make sure the query is a `productVariants` query
        let var_str = match variants_query.query_type() {
            ProductVariantQueryType::ProductVariant => panic!("ERROR REPLACE THIS"),
            ProductVariantQueryType::ProductVariants(conn) => match conn {
                ProductsConnection::First(n) => {
                    format!(
                        "variants(first: {}) {{ edges {{ node {{ {} }} }} }}",
                        n,
                        variants_query.fields().join("\n,")
                    )
                }
                ProductsConnection::Last(n) => {
                    format!(
                        "variants(last: {}) {{ edges {{ node {{ {} }} }} }}",
                        n,
                        variants_query.fields().join("\n,")
                    )
                }
            },
        };

        // let var_str = format!(
        //     "variants(first: {}) {{ edges {{ node {{ {} }} }} }}",
        //     first,
        //     variants_query.fields().join("\n,")
        // );

        self.fields.push(var_str);
        self
    }

    pub(crate) async fn build(self, config: ShopifyConfig) -> ShopifyResult<Product> {
        let fields = self.fields.join("\n,");

        let query = match self.query_type {
            ProductQueryType::Product => {
                format!(
                    "query {{ product(id: \"{}\") {{ {} }} }}",
                    self.id.inner(),
                    fields
                )
            }
        };

        let res = run_query(config, query).await?;
        match res.data {
            ResponseTypes::Product(p) => Ok(p),

            _ => unreachable!(), // FIX: Replace this with an Error
        }
    }

    pub(crate) fn fields(&self) -> &[String] {
        self.fields.as_ref()
    }
}
