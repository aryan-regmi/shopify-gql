#![allow(unused)]

use crate::{
    common::{Id, Money},
    utils::{run_query, ResponseTypes, ShopifyConfig, ShopifyResult},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductVariant {
    id: Id,
    compare_at_price: Option<Money>,
}

impl ProductVariant {
    pub(crate) fn from_query(id: Id) -> ProductVariantQueryBuilder {
        let mut fields = Vec::new();
        fields.push("id".into());

        ProductVariantQueryBuilder { id, fields }
    }

    pub(crate) fn id(&self) -> &Id {
        &self.id
    }

    pub(crate) fn compare_at_price(&self) -> Option<&Money> {
        self.compare_at_price.as_ref()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductVariantQueryBuilder {
    id: Id,
    fields: Vec<String>,
}

impl ProductVariantQueryBuilder {
    // pub(crate) async fn build(self, config: ShopifyConfig) -> ShopifyResult<ProductVariant> {
    //     let fields = self.fields.join("\n,");
    //
    //     let query = format!(
    //         "query {{ product(id: \"{}\") {{ {} }} }}",
    //         self.id.inner(),
    //         fields
    //     );
    //
    //     let res = run_query(config, query).await?;
    //     match res.data {
    //         ResponseTypes::ProductVariant(v) => Ok(v),
    //
    //         _ => unreachable!(), // FIX: Replace this with an Error
    //     }
    // }

    pub(crate) fn compare_at_price(mut self) -> Self {
        self.fields.push("compareAtPrice".into());
        self
    }

    pub(crate) fn fields(&self) -> &[String] {
        self.fields.as_ref()
    }
}
