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
use axum::Extension;

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
            create_schedule_chirho,
            delete_schedule_chirho,
            get_schedules_chirho,
        },
    },
    public_chirho::{
        //create_signup_chirho,
        get_public_schedules_chirho,
        get_upcoming_schedules_chirho,
        get_church_by_token_chirho,
    },
};
use crate::handlers_chirho::public_chirho::create_signup_chirho;

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
            "http://localhost:5174".parse().unwrap(),
            "http://127.0.0.1:5174".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(),
            "http://127.0.0.1:5173".parse().unwrap(),
        ]))
        .allow_methods(AllowMethods::list(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
            header::ACCESS_CONTROL_REQUEST_METHOD,
            header::ACCESS_CONTROL_REQUEST_HEADERS,
        ])
        .allow_credentials(true)
        .expose_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
        ]);

    // Create router
    let admin_router_chirho = Router::new()
        // Admin routes
        .route("/api_chirho/admin_chirho/auth_chirho/login_chirho", post(login_chirho))
        .route("/api_chirho/admin_chirho/auth_chirho/logout_chirho", post(logout_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho", get(get_continents_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho/{id_chirho}", get(get_continent_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho", post(create_continent_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho/{id_chirho}", put(update_continent_chirho))
        .route("/api_chirho/admin_chirho/continents_chirho/{id_chirho}", delete(delete_continent_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho", get(get_churches_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho/{id_chirho}", get(get_church_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho", post(create_church_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho/{id_chirho}", put(update_church_chirho))
        .route("/api_chirho/admin_chirho/churches_chirho/{id_chirho}", delete(delete_church_chirho))
        .route("/api_chirho/admin_chirho/schedule_chirho", get(get_schedules_chirho))
        .route("/api_chirho/admin_chirho/schedule_chirho/assign_chirho", post(create_schedule_chirho))
        .route("/api_chirho/admin_chirho/schedule_chirho/unassign_chirho/{schedule_id_chirho}", delete(delete_schedule_chirho))
        // Auth routes
        .route("/api_chirho/admin_chirho/auth_chirho/logout_chirho", delete(logout_chirho));        

    // Public routes
    let public_router_chirho = Router::new()
        .route("/api_chirho/public_chirho/church_chirho/{church_token_chirho}", get(get_church_by_token_chirho))
        .route("/api_chirho/public_chirho/church_chirho/{church_token_chirho}/assign_to_schedule_chirho/{schedule_id_chirho}", post(create_signup_chirho))
        .route("/api_chirho/public_chirho/schedules_chirho/upcoming_chirho", get(get_upcoming_schedules_chirho));


    // Combine all routers
    let app_chirho = Router::new()
        .merge(admin_router_chirho)
        .merge(public_router_chirho)
        .layer(cors_chirho)
        .with_state(pool_chirho);

    // Start server
    let addr_chirho = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener_chirho = TcpListener::bind(addr_chirho).await.unwrap();

    println!("Server listening on {}", addr_chirho);
    axum::serve(listener_chirho, app_chirho).await.unwrap();
}
