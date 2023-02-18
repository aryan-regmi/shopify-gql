mod products;

mod utils;

mod common;

#[cfg(test)]
mod tests {
    use crate::{
        common::{Id, Money, WeightUnit},
        products::{
            product::{Product, ProductQueryBuilder, ProductStatus},
            product_variant::{ProductVariant, ProductVariantQueryBuilder},
            ProductsConnection,
        },
        utils::{run_query, QueryResponse, ShopifyConfig, ShopifyResult},
    };
    use std::any::{Any, TypeId};

    #[tokio::test]
    async fn test_connection() -> ShopifyResult<()> {
        let res = run_query(
            ShopifyConfig::from_env()?,
            "query { product(id: \"gid://shopify/Product/7343141159089\") { id } }".into(),
        )
        .await?;

        // dbg!(&res.ty);
        assert_eq!(res.type_id(), TypeId::of::<QueryResponse>());

        Ok(())
    }

    #[tokio::test]
    async fn can_run_product_query() -> ShopifyResult<()> {
        let config = ShopifyConfig::from_env()?;
        let prod = ProductQueryBuilder::product(Id::product("7343141159089")?)
            .status()
            .vendor()
            .title()
            .build(config)
            .await?;

        dbg!(&prod);

        assert_eq!(prod.id(), &Id::product("7343141159089")?);
        assert_eq!(prod.status().unwrap(), &ProductStatus::DRAFT);
        assert_eq!(prod.vendor().unwrap(), "TEST");
        assert_eq!(prod.title().unwrap(), "MY TITLE");

        Ok(())
    }

    #[tokio::test]
    async fn can_run_product_query_with_variants() -> ShopifyResult<()> {
        let config = ShopifyConfig::from_env()?;
        let prod = ProductQueryBuilder::product(Id::product("7343141159089")?)
            .status()
            .vendor()
            .title()
            .variants(
                ProductVariantQueryBuilder::product_variants(ProductsConnection::First(1))
                    .compare_at_price()
                    .inventory_quantity()
                    .price()
                    .product(ProductQueryBuilder::product(Id::default()).vendor())
                    .sku()
                    .title()
                    .weight()
                    .weight_unit(),
            )
            .build(config)
            .await?;

        dbg!(&prod);

        assert_eq!(prod.id(), &Id::product("7343141159089")?);
        assert_eq!(prod.status().unwrap(), &ProductStatus::DRAFT);
        assert_eq!(prod.vendor().unwrap(), "TEST");
        assert_eq!(prod.title().unwrap(), "MY TITLE");

        let var = prod.variants().unwrap().get_node(0);
        assert_eq!(var.id(), &Id::product_variant("42235355201713")?);
        assert_eq!(var.compare_at_price(), Some(&Money(22.0)));
        assert_eq!(var.inventory_quantity(), Some(10));
        assert_eq!(var.price(), Some(&Money(42.99)));
        assert_eq!(var.sku(), Some(&"12345-red".into()));
        assert_eq!(var.title(), Some(&"Red".into()));
        assert_eq!(var.weight(), Some(10.0));
        assert_eq!(var.weight_unit(), Some(&WeightUnit::POUNDS));

        Ok(())
    }

    #[tokio::test]
    async fn can_update_product() -> ShopifyResult<()> {
        let config = ShopifyConfig::from_env()?;

        let prod = ProductQueryBuilder::product_update(Id::product("7343141159089")?)
            .update_title("MY TITLE")
            .update_vendor("TEST")
            .update_status(ProductStatus::DRAFT)
            .build(config)
            .await?;

        assert_eq!(prod.id(), &Id::product("7343141159089")?);

        Ok(())
    }
}
