use crate::{
    config_chirho::ConfigChirho,
    error_chirho::AppErrorChirho,
    middleware_chirho::create_token_chirho,
    models_chirho::application_administrator_chirho::{LoginRequestChirho as AdminLoginRequestChirho, LoginResponseChirho as AdminLoginResponseChirho},
};
// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use bcrypt::verify;
use sqlx::{MySql, Pool};

pub async fn login_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    State(config_chirho): State<ConfigChirho>,
    Json(login_chirho): Json<AdminLoginRequestChirho>,
) -> Result<Json<AdminLoginResponseChirho>, AppErrorChirho> {
    let admin_chirho = sqlx::query_as!(
        crate::models_chirho::application_administrator_chirho::ApplicationAdministratorChirho,
        r#"
        SELECT admin_id_chirho, username_chirho, password_representation_chirho, created_timestamp_chirho
        FROM application_administrators_chirho
        WHERE username_chirho = ?
        "#,
        login_chirho.username_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    if !verify(&login_chirho.password_chirho, &admin_chirho.password_representation_chirho)? {
        println!("Invalid password Aleluya");
        return Err(AppErrorChirho::Validation("Invalid credentials".to_string()));
    }
    println!("Admin chirho password representation chirho");

    let token_chirho = create_token_chirho(
        admin_chirho.admin_id_chirho.to_string(),
        "admin_chirho".to_string(),
        &config_chirho,
    )?;

    Ok(Json(AdminLoginResponseChirho { token_chirho: token_chirho }))
}

pub async fn logout_chirho(jar_chirho: CookieJar) -> impl IntoResponse {
    let jar_chirho = jar_chirho.remove(Cookie::build("auth_token_chirho").build());
    (jar_chirho, StatusCode::OK)
} 