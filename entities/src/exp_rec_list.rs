use sea_orm::DerivePrimaryKey;
use sea_orm::EnumIter;
use sea_orm::DeriveEntityModel;
use sea_orm::prelude::{Decimal, Uuid};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(schema_name = "finance", table_name = "expense_with_receipt")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub expense_date: chrono::DateTime<chrono::FixedOffset>,
    pub expense_amount: Decimal,
    pub total_daily_sallary: Decimal,
    pub balance: Decimal
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
