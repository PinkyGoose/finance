use sea_orm::prelude::{Decimal, Uuid};
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DeriveIntoActiveModel, DeriveRelation, EnumIter,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(schema_name = "security", table_name = "user")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,
    pub login: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub name: String,
    pub login: String,
    pub password: String,
}

impl ActiveModelBehavior for ActiveModel {}
