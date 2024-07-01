use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Debug, ToSchema)]
pub struct CreatedEntity{
    uuid: Uuid,
}
impl CreatedEntity {
    pub fn new(uuid: Uuid)-> Self{
        Self{uuid}
    }
}