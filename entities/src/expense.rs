//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(schema_name = "finance", table_name = "expense")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub value_sum: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateExpense {
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub value_sum: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExpense {
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub value_sum: Decimal,
}

impl ActiveModelBehavior for ActiveModel {}
