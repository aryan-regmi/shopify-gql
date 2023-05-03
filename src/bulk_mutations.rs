use serde::Deserialize;

use crate::{common::Id, utils::ResponseTypes};

#[derive(Debug, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
enum BulkOperationStatus {
    CANCELED,
    CANCELING,
    COMPLETED,
    CREATED,
    EXPIRED,
    FAILED,
    RUNNING,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BulkOperation {
    id: Id,
    status: BulkOperationStatus,
    url: Option<String>,
}

#[allow(unused)]
struct BulkOperationBuilder {
    id: Id,
    status: BulkOperationStatus,
    url: Option<String>,
}

#[allow(unused)]
impl BulkOperationBuilder {
    fn query(query_type: ResponseTypes) -> Self {
        // TODO: Need to construct query, but remove the numbers in `first` or `last`
        match query_type {
            ResponseTypes::Product(_) => todo!(),
            ResponseTypes::ProductVariant(_) => todo!(),
            _ => unreachable!(),
        }

        todo!()
    }

    fn mutation() -> Self {
        todo!()
    }

    fn build() -> BulkOperation {
        todo!()
    }
}
