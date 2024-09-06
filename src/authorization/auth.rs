use crate::utils::{CreatedEntity, Error};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::{Extension, Json};
use entities::user::CreateUser;
use entities::user::Entity as UserEntity;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};
pub async fn create_user(
    Extension(ref pool): Extension<DatabaseConnection>,
    Extension(ref argon): Extension<Argon2<'static>>,
    Json(mut payload): Json<CreateUser>,
) -> Result<Json<CreatedEntity>, Error> {
    let password = payload.password.as_mut();

    let salt = SaltString::generate(&mut OsRng);
    let hash = argon
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::InvalidData("невозможно сгенерировать пароль".into()))?;

    let password = hash.to_string();
    payload.password = password;
    let model = payload.into_active_model();

    let user = UserEntity::insert(model)
        .exec_with_returning(pool)
        .await
        .map_err(Error::DatabaseInternal)
        .map(|user| Json(CreatedEntity::new(user.id)))?;

    Ok(Json(user.0))
}
