// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
pub use tokio::net::TcpListener;
use tower_http::cors::{AllowMethods, AllowOrigin};

// <-- Import TcpListener from Tokio

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use axum::http::{header, Method};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config_chirho;
mod db_chirho;
mod error_chirho;
mod handlers_chirho;
mod middleware_chirho;
mod models_chirho;

use config_chirho::ConfigChirho;
use db_chirho::init_db_chirho;
use handlers_chirho::{
    admin_chirho::{
        auth_chirho::{
            login_chirho,
            logout_chirho,
        },
        churches_chirho::{
            create_church_chirho,
            delete_church_chirho,
            get_church_chirho,
            get_churches_chirho,
            update_church_chirho,
        },
        continents_chirho::{
            create_continent_chirho,
            delete_continent_chirho,
            get_continent_chirho,
            get_continents_chirho,
            update_continent_chirho,
        },
        schedules_chirho::{
            create_schedule_chirho
            ,
            delete_schedule_chirho
            ,
            get_schedules_chirho,
        },
    },
    public_chirho::{
        create_signup_chirho,
        get_public_schedules_chirho,
    },
};

#[tokio::main]
async fn main() {

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv().ok();

    // Initialize configuration
    let config_chirho = ConfigChirho::from_env();

    // Initialize database
    let pool_chirho = init_db_chirho(&config_chirho).await.expect("Failed to initialize database");

    // Set up CORS
    let cors_chirho = CorsLayer::new()
        .allow_origin(AllowOrigin::list(vec![
            "http://localhost:5173".parse().unwrap(),
            "http://example.com".parse().unwrap(),
        ]))
        .allow_methods(AllowMethods::list(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ]))
        .allow_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .allow_credentials(true);

    // Create router
    let app_chirho = Router::new()
        // Admin routes
        .route("/api_chirho/admin_chirho/auth_chirho/login_chirho", post(login_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho", get(get_continents_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho/:id", get(get_continent_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho", post(create_continent_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho/:id", put(update_continent_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho/:id", delete(delete_continent_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho", get(get_churches_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho/:id", get(get_church_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho", post(create_church_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho/:id", put(update_church_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho/:id", delete(delete_church_chirho))
        .route("/api_chirho/admin_chirho/schedule_chirho", get(get_schedules_chirho))
        .route("/api_chirho/admin_chirho/schedule_chirho/assign_chirho", post(create_schedule_chirho))
        .route("/api_chirho/admin_chirho/schedule_chirho/unassign_chirho", delete(delete_schedule_chirho))
        // Auth routes
        .route("/api_chirho/admin_chirho/auth_chirho/logout_chirho", delete(logout_chirho))
        // Public routes
        .route("/api_chirho/public_chirho/church_chirho/:token_chirho/schedule_chirho/:date_chirho", get(get_public_schedules_chirho))
        .route("/api_chirho/public_chirho/church_chirho/:token_chirho/signup_chirho", post(create_signup_chirho))
        .layer(cors_chirho)
        .with_state(pool_chirho);

    // Start server
    let addr_chirho = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener_chirho = TcpListener::bind(addr_chirho).await.unwrap();

    println!("Server listening on {}", addr_chirho);
    axum::serve(listener_chirho, app_chirho.into_make_service())
        .await
        .unwrap();
}
