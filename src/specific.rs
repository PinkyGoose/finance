use axum::Router;

mod expence;



pub fn router() -> Router {

    let money = Router::new()
        .route("/arm", post(arm::create_arm).get(arm::get_arms))
        .route(
            "/arm/:id",
            get(arm::get_arm).put(arm::edit_arm).delete(arm::delete_arm),
        )
        .route("/arm/updates", get(arm::get_arm_updates))
        .route("/arm_list", get(arm::get_arm_list));


    let security_router = money;

    Router::new()
        .nest("/system", system_router)
        .nest("/ui", ui_router)
        .nest("/health", health_router)
        .nest("/service", service_router)
        .nest("/events", events_router)
        .nest("/security", security_router)
        .nest("/updates", updates_router)
}
