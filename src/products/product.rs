#![allow(unused)]

use std::{collections::HashMap, marker::PhantomData};

use crate::{
    common::{Edges, Id, Node},
    utils::{run_query, ResponseTypes, ShopifyConfig, ShopifyGqlError, ShopifyResult},
};
use serde::{de::IntoDeserializer, Deserialize};

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
    ProductUpdate(Id),
}

// NOTE: This needs to be updated anytime a new field is added to `Product`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductQueryBuilder {
    id: Id,
    fields: HashMap<String, PhantomData<u8>>,
    inputs: Option<HashMap<String, PhantomData<u8>>>,
    query_type: ProductQueryType,
}

impl ProductQueryBuilder {
    pub(crate) fn product(id: Id) -> Self {
        let mut fields = HashMap::new();
        fields.insert("id".into(), PhantomData);

        let query_type = ProductQueryType::Product;

        ProductQueryBuilder {
            id,
            fields,
            inputs: None,
            query_type,
        }
    }

    ///**NOTE:** Only call the `update_` methods on the returned builder.
    pub(crate) fn product_update(id: Id) -> Self {
        let mut fields = HashMap::new();
        fields.insert("id".into(), PhantomData);

        let mut inputs = Some(HashMap::new());
        inputs
            .as_mut()
            .unwrap()
            .insert(format!("id: \"{}\"", id.inner()), PhantomData);

        let query_type = ProductQueryType::ProductUpdate(id.clone());

        ProductQueryBuilder {
            id,
            fields,
            inputs,
            query_type,
        }
    }

    pub(crate) fn status(mut self) -> Self {
        self.fields.insert("status".into(), PhantomData);
        self
    }

    pub(crate) fn update_status(mut self, status: ProductStatus) -> Self {
        let status = format!("status: {:?}", status);

        self.inputs.as_mut().unwrap().insert(status, PhantomData);
        self
    }

    pub(crate) fn vendor(mut self) -> Self {
        self.fields.insert("vendor".into(), PhantomData);
        self
    }

    pub(crate) fn update_vendor(mut self, vendor: &str) -> Self {
        let vendor = format!("vendor: {:?}", vendor);

        self.inputs.as_mut().unwrap().insert(vendor, PhantomData);
        self
    }

    pub(crate) fn title(mut self) -> Self {
        self.fields.insert("title".into(), PhantomData);
        self
    }

    pub(crate) fn update_title(mut self, title: &str) -> Self {
        let title = format!("title: {:?}", title);

        self.inputs.as_mut().unwrap().insert(title, PhantomData);
        self
    }

    pub(crate) fn variants(mut self, variants_query: ProductVariantQueryBuilder) -> Self {
        // Make sure the query is a `productVariants` query
        let var_str = match variants_query.query_type() {
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

            _ => panic!("ERROR REPLACE THIS"),
        };

        self.fields.insert(var_str, PhantomData);
        self
    }

    pub(crate) fn fields(&self) -> Vec<&str> {
        self.fields.keys().map(|v| v.as_str()).collect()
    }

    pub(crate) fn inputs(&self) -> Option<Vec<&str>> {
        self.inputs
            .as_ref()
            .map_or(None, |m| Some(m.keys().map(|v| v.as_str()).collect()))
    }

    pub(crate) async fn build(self, config: ShopifyConfig) -> ShopifyResult<Product> {
        let fields = self.fields().join("\n,");
        let inputs = self.inputs();

        let query = match &self.query_type {
            ProductQueryType::Product => {
                format!(
                    "query {{ product(id: \"{}\") {{ {} }} }}",
                    self.id.inner(),
                    fields
                )
            }

            // TODO: Handle user errors
            ProductQueryType::ProductUpdate(id) => {
                format!(
                    "mutation {{ productUpdate(input: {{ {} }}) {{ product {{ {} }} }}  }}",
                    inputs.unwrap().join("\n,"),
                    fields
                )
            }
        };

        let res = run_query(config, query).await?;
        match res.data {
            ResponseTypes::Product(p) => Ok(p),

            ResponseTypes::ProductUpdate { product } => Ok(product),

            _ => Err(ShopifyGqlError::ResponseError(format!("{:?}", res))),
        }
    }
}
