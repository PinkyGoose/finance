use std::time::SystemTime;
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, TransactionError};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use entities::{expense::{CreateExpense, Entity as ExpenseEntity, Model as Expense, UpdateExpense}, expense};
use axum::{response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Decimal;
use entities::expense::Entity;

#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseQuery{

}
pub async fn root() -> &'static str {
    "Hello, World!"
}
#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
pub struct CreatedEntity{
    uuid: Uuid,
}
impl ErrorResponse{
    pub fn new(mes: &str)->Self{
        Self{error: mes.parse().unwrap() }
    }
}
impl CreatedEntity {
    pub fn new(uuid: Uuid)-> Self{
        Self{uuid}
    }
}
#[utoipa::path(post, path = "/expense/expense",
request_body = CreateExpense,
responses(
(status = 200, description = "Успешное создание Затрат", body = Expense),
(status = 500, description = "Ошибка исполнения запроса")
)
)]
pub(crate) async fn create_expense(
    Extension(ref pool): Extension<DatabaseConnection>,
    Json(payload): Json<CreateExpense>,
)-> Result<Json<CreatedEntity>, Error>{
    let arm = ExpenseEntity::insert(payload.into_active_model())
        .exec_with_returning(pool)
        .await
        .map_err(Error::DatabaseInternal)
        .map(|arm| Json(CreatedEntity::new(arm.id)))?;
    Ok(arm)
}


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("неверный UUID")]
    InvalidUuid,
    #[error("не найдено (что?)")]
    NotFound,
    #[error("ошибка внутри базы данных: {0}")]
    DatabaseInternal(#[from] sea_orm::error::DbErr),
    #[error("неверные данные: {0}")]
    InvalidData(String),
    #[error("пользователь не авторизован")]
    Unauthorized,
    #[error("у сущности есть активные зависимости")]
    NotEmpty,
    #[error("отсутствует подключение к реестру компонентов")]
    NoComponentRegistry,
    #[error("ошибка валидации входящих данных: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("пользователь не имеет достаточных прав")]
    Forbidden,
}

impl From<TransactionError<sea_orm::error::DbErr>> for Error {
    fn from(value: TransactionError<sea_orm::error::DbErr>) -> Self {
        match value {
            TransactionError::Connection(err) => Self::DatabaseInternal(err),
            TransactionError::Transaction(err) => Self::DatabaseInternal(err),
        }
    }
}

impl From<Error> for serde_json::Value {
    fn from(val: Error) -> Self {
        #[rustfmt::skip] // некрасиво форматирует
        let (code, descr) = match &val {
            Error::InvalidUuid                                           => ("E_INVALID_UUID",        String::default()),
            Error::NotFound                                              => ("E_OBJECT_NOT_FOUND",    String::default()),
            Error::InvalidData(desc)                                     => ("E_INVALID_DATA",        desc.clone()),
            Error::Unauthorized                                          => ("E_USER_UNAUTHORIZED",   "пользователь не авторизован".into()),
            Error::NotEmpty                                              => ("E_ENTITY_NOT_EMPTY",    "у сущности есть активные зависимости".into()),
            Error::NoComponentRegistry                                   => ("E_NO_COMPONENT_REGISTRY", "отсутствует подключение к реестру компонентов".into()),
            Error::Validation(inner)                                     => ("E_INVALID_INPUT", format!("ошибка валидации входящих данных: {inner}")),
            Error::Forbidden                                             => ("E_FORBIDDEN", "пользователь не имеет достаточных прав".into()),
            Error::DatabaseInternal(sea_orm::DbErr::Conn(err))           => ("E_DATABASE_CONNECTION", err.to_string()),
            Error::DatabaseInternal(sea_orm::DbErr::Exec(err))           => ("E_DATABASE_EXEC",       err.to_string()),
            Error::DatabaseInternal(sea_orm::DbErr::Query(err))          => ("E_DATABASE_QUERY",      err.to_string()),
            Error::DatabaseInternal(sea_orm::DbErr::RecordNotFound(err)) => ("E_DATABASE_NOTFOUND",   err.to_string()),
            Error::DatabaseInternal(sea_orm::DbErr::Custom(err))         => ("E_DATABASE_CUSTOM",     err.to_string()),
            Error::DatabaseInternal(sea_orm::DbErr::Type(err))           => ("E_DATABASE_TYPE",       err.to_string()),
            Error::DatabaseInternal(sea_orm::DbErr::Json(err))           => ("E_DATABASE_JSON",       err.to_string()),
            Error::DatabaseInternal(sea_orm::DbErr::Migration(err))      => ("E_DATABASE_MIGRATION",  err.to_string()),
            #[allow(unreachable_patterns)] // для будущего расширения вариантов ошибок
            Error::DatabaseInternal(_)                                   => ("E_DATABASE_UNKNOWN",    String::default()),
        };

        serde_json::json!({
            "code": code,
            "description": descr
        })
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (code, text): (_, serde_json::Value) = match self {
            Error::InvalidUuid => (StatusCode::BAD_REQUEST, self.into()),
            Error::NotFound => (StatusCode::NOT_FOUND, self.into()),
            Error::DatabaseInternal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.into()),
            Error::InvalidData(_) => (StatusCode::BAD_REQUEST, self.into()),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, self.into()),
            Error::NotEmpty => (StatusCode::CONFLICT, self.into()),
            Error::NoComponentRegistry => (StatusCode::INTERNAL_SERVER_ERROR, self.into()),
            Error::Validation(_) => (StatusCode::BAD_REQUEST, self.into()),
            Error::Forbidden => (StatusCode::FORBIDDEN, self.into()),
        };

        (code, text.to_string()).into_response()
    }
}
