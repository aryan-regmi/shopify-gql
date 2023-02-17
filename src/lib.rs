mod products;

mod utils;

mod common;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        common::{Id, Money},
        products::{
            product::{Product, ProductStatus},
            product_variant::ProductVariant,
        },
        utils::{run_query, ShopifyConfig, ShopifyResult},
    };

    #[tokio::test]
    #[ignore]
    async fn test_connection() -> ShopifyResult<()> {
        let res = run_query(
            ShopifyConfig::from_env()?,
            "query { product(id: \"gid://shopify/Product/7343141159089\") { id } }".into(),
        )
        .await?;

        dbg!(&res);

        Ok(())
    }

    #[tokio::test]
    async fn can_run_product_query() -> ShopifyResult<()> {
        let config = ShopifyConfig::from_env()?;
        let prod = Product::from_query(Id::product("7343141159089")?)
            .status()
            .vendor()
            .title()
            .variants(
                10,
                ProductVariant::from_query(Id::default()).compare_at_price(),
            )
            .build(config)
            .await?;

        dbg!(&prod);

        assert_eq!(prod.id(), &Id::product("7343141159089")?);
        assert_eq!(prod.status().unwrap(), &ProductStatus::ACTIVE);
        assert_eq!(prod.vendor().unwrap(), "NTP Dev Env");
        assert_eq!(prod.title().unwrap(), "Test Prod 1");

        let var = prod.variants().unwrap().get_node(0);
        assert_eq!(var.id(), &Id::product_variant("42235355201713")?);
        assert_eq!(var.compare_at_price(), Some(&Money(22.0)));

        Ok(())
    }
}
