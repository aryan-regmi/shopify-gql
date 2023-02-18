#![allow(unused)]

use crate::{
    common::{Id, Money, WeightUnit},
    utils::{
        run_query, ResponseTypes, ShopifyConfig, ShopifyConnection, ShopifyGqlError, ShopifyResult,
    },
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
    ProductVariantUpdate(Id),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductVariantQueryBuilder {
    id: Id,
    fields: Vec<String>,
    inputs: Option<Vec<String>>,
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
            inputs: None,
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
            inputs: None,
            query_type,
        }
    }

    ///**NOTE:** Only call the `update_` methods on the returned builder.
    pub(crate) fn product_variant_update(id: Id) -> Self {
        let mut fields = Vec::new();
        fields.push("id".into());

        let mut inputs = Some(Vec::new());
        inputs
            .as_mut()
            .unwrap()
            .push(format!("id: \"{}\"", id.inner()));

        let query_type = ProductVariantQueryType::ProductVariantUpdate(id.clone());

        ProductVariantQueryBuilder {
            id,
            fields,
            inputs,
            query_type,
        }
    }

    pub(crate) fn compare_at_price(mut self) -> Self {
        self.fields.push("compareAtPrice".into());
        self
    }

    pub(crate) fn update_compare_at_price(mut self, compare_at_price: Money) -> Self {
        let compare_at_price = format!("compareAtPrice: {}", compare_at_price.0.to_string());

        self.inputs.as_mut().unwrap().push(compare_at_price);
        self
    }

    pub(crate) fn inventory_quantity(mut self) -> Self {
        self.fields.push("inventoryQuantity".into());
        self
    }

    // TODO: Update `Id` to return location_id, change `location_id` to Id from &str
    pub(crate) fn update_inventory_quantities(
        mut self,
        inventory_quantities: Vec<(i32, Id)>,
    ) -> Self {
        let mut body_str = String::new();
        for v in inventory_quantities {
            body_str.push_str(&format!(
                "{{ availableQuantity: {}, locationId: \"{}\" }},",
                v.0,
                v.1.inner()
            ));
        }

        let inventory_quantity = format!("inventoryQuantities: [{}]", body_str);

        self.inputs.as_mut().unwrap().push(inventory_quantity);
        self
    }

    pub(crate) fn price(mut self) -> Self {
        self.fields.push("price".into());
        self
    }

    pub(crate) fn update_price(mut self, price: Money) -> Self {
        let price = format!("price: {}", price.0.to_string());

        self.inputs.as_mut().unwrap().push(price);
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

    pub(crate) fn update_sku(mut self, sku: &str) -> Self {
        let sku = format!("sku: \"{}\"", sku);

        self.inputs.as_mut().unwrap().push(sku);
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

    pub(crate) fn update_weight(mut self, weight: f64) -> Self {
        let weight = format!("weight: {}", weight);

        self.inputs.as_mut().unwrap().push(weight);
        self
    }

    pub(crate) fn weight_unit(mut self) -> Self {
        self.fields.push("weightUnit".into());
        self
    }

    pub(crate) fn update_weight_unit(mut self, weight_unit: WeightUnit) -> Self {
        let weight_unit = format!("weightUnit: {:?}", weight_unit);

        self.inputs.as_mut().unwrap().push(weight_unit);
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

            // TODO: Handle user errors
            ProductVariantQueryType::ProductVariantUpdate(id) => {
                format!(
                    "mutation {{ productVariantUpdate(input: {{ {} }}) {{ productVariant {{ {} }} }}  }}",
                    self.inputs.unwrap().join("\n,"),
                    fields
                )
            }
        };

        let res = run_query(config, query).await?;
        match res.data {
            ResponseTypes::ProductVariant(v) => Ok(v),

            ResponseTypes::ProductVariantUpdate { product_variant } => Ok(product_variant),

            _ => Err(ShopifyGqlError::ResponseError(format!("{:?}", res))),
        }
    }
}
