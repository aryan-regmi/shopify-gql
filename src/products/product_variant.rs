#![allow(unused)]

use serde::Deserialize;

use super::Product;
use crate::common::{Id, InventoryItem, Money, WeightUnit};

/// Represents a product variant.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductVariant {
    /// A globally-unique identifier.
    id: Id,

    /// The compare-at price of the variant in the default shop currency.
    compare_at_price: Option<Money>,

    /// The inventory item, which is used to query for inventory information.
    inventory_item: InventoryItem,

    /// The total sellable quantity of the variant.
    inventory_quantity: Option<i32>,

    /// The price of the product variant in the default shop currency.
    price: Money,

    /// The product that this variant belongs to.
    product: Box<Product>,

    /// An identifier for the product variant in the shop. Required in order to connect to a fulfillment service.
    sku: Option<String>,

    /// The title of the product variant.
    title: String,

    /// The weight of the product variant in the unit system specified with weight_unit.
    weight: Option<f32>,

    /// The unit of measurement that applies to the product variant's weight.
    weight_unit: WeightUnit,
}
