#![allow(unused)]

use serde::Deserialize;

use crate::products::ProductVariant;

/// A globally-unique identifier.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Id(String);
impl Id {
    pub(crate) fn product(id: &str) -> Self {
        let id = format!("gid://shopify/Product/{id}");

        Self(id)
    }

    pub(crate) fn product_variant(id: &str) -> Self {
        let id = format!("gid://shopify/ProductVariant/{id}");

        Self(id)
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Money(String);

/// The three-letter currency codes that represent the world currencies used in stores.
#[derive(Debug, Deserialize)]
enum CurrencyCode {
    /// Canadian Dollars (CAD).
    CAD,

    /// United Kingdom Pounds (GBP).
    GBP,

    /// United States Dollars (USD).
    USD,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MoneyV2 {
    /// Decimal money amount.
    amount: f64,

    /// Currency of the money.
    currency_code: CurrencyCode,
}

/// Represents the goods available to be shipped to a customer.
/// It holds essential information about the goods, including SKU and whether it is tracked.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InventoryItem {
    /// A globally-unique identifier.
    id: Id,

    /// Inventory item SKU.
    sku: Option<String>,

    /// Unit cost associated with the inventory item.
    /// Note: the user must have "View product costs" permission granted in
    /// order to access this field once product granular permissions are enabled.
    unit_cost: Option<MoneyV2>,

    /// The variant that owns this inventory item.
    variant: Box<ProductVariant>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum WeightUnit {
    /// Metric system unit of mass.
    GRAMS,

    /// 1 kilogram equals 1000 grams.
    KILOGRAMS,

    /// Imperial system unit of mass.
    OUNCES,

    /// 1 pound equals 16 ounces.
    POUNDS,
}
