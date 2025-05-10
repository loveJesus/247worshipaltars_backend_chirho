// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
#![allow(unused_imports)]
pub use tokio::net::TcpListener;
use tower_http::cors::{AllowMethods, AllowOrigin};
use std::io::Write;

#[cfg(target_os = "openbsd")]
use openbsd::unveil;

// <-- Import TcpListener from Tokio
use rust_embed::RustEmbed;
use axum_embed::{FallbackBehavior, ServeEmbed};

use axum::{
    routing::{delete, get, post, put},
    Router,
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::IntoResponse,
    extract::Path,
    extract::State,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use std::path::PathBuf;
use axum::http::{header, Method};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::Extension;
use std::sync::Arc;
use axum_server::tls_rustls::RustlsConfig;
use tokio::sync::broadcast;
use serde_json::json;
use sqlx::MySqlPool;

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
        pdf_file_chirho::{
            upload_pdf_file_chirho,
            get_all_pdf_files_chirho,
            delete_pdf_file_chirho,
        },
    },
    public_chirho::{
        //create_signup_chirho,
        get_public_schedules_chirho,
        get_upcoming_schedules_chirho,
        get_church_by_token_chirho,
        serve_pdf_file_chirho,
    },
};
use crate::handlers_chirho::public_chirho::{create_signup_chirho, get_schedule_signups_chirho, delete_signup_chirho};

#[derive(Clone)]
struct AppStateChirho {
    tx: broadcast::Sender<SignupEventChirho>,
}

#[derive(Clone, Debug, serde::Serialize)]
struct SignupEventChirho {
    church_id_chirho: String,
    schedule_id_chirho: String,
    event_type_chirho: String,
    data_chirho: serde_json::Value,
}

async fn websocket_handler_chirho(
    ws_chirho: WebSocketUpgrade,
    State((pool_chirho, state_chirho)): State<(MySqlPool, Arc<AppStateChirho>)>,
    Path(church_id_chirho): Path<String>,
) -> impl IntoResponse {
    eprintln!("[WebSocket] Handler called for church_id: {}", church_id_chirho.clone());
    std::io::stderr().flush().unwrap();
    
    let response_chirho = ws_chirho.on_upgrade(move |socket_chirho| {
        eprintln!("[WebSocket] Upgrade successful for church_id: {}", church_id_chirho.clone());
        std::io::stderr().flush().unwrap();
        handle_socket_chirho(socket_chirho, state_chirho, church_id_chirho)
    });
    
    //eprintln!("[WebSocket] Returning response for church_id: {}", church_id_chirho.clone());
    std::io::stderr().flush().unwrap();
    response_chirho
}

async fn handle_socket_chirho(
    mut socket_chirho: WebSocket,
    state_chirho: Arc<AppStateChirho>,
    church_id_chirho: String,
) {
    eprintln!("[WebSocket] Connection established for church_id: {}", church_id_chirho);
    std::io::stderr().flush().unwrap();
    
    let mut rx_chirho = state_chirho.tx.subscribe();
    eprintln!("[WebSocket] Subscribed to broadcast channel for church_id: {}", church_id_chirho);
    std::io::stderr().flush().unwrap();
    
    // Send initial message to confirm connection
    let initial_message_chirho = serde_json::json!({
        "type_chirho": "connected",
        "church_id_chirho": church_id_chirho
    });
    
    match socket_chirho.send(Message::Text(initial_message_chirho.to_string().into())).await {
        Ok(_) => {
            eprintln!("[WebSocket] Initial message sent successfully to church_id: {}", church_id_chirho);
            std::io::stderr().flush().unwrap();
        }
        Err(e_chirho) => {
            eprintln!("[WebSocket] Failed to send initial message to church_id {}: {:?}", church_id_chirho, e_chirho);
            std::io::stderr().flush().unwrap();
            return;
        }
    }
    
    loop {
        tokio::select! {
            msg = rx_chirho.recv() => {
                match msg {
                    Ok(event_chirho) => {
                        eprintln!("[WebSocket] Received broadcast event for church_id {}: {:?}", church_id_chirho, event_chirho);
                        std::io::stderr().flush().unwrap();
                        if event_chirho.church_id_chirho == church_id_chirho {
                            // Format the event with a consistent structure
                            let formatted_event_chirho = serde_json::json!({
                                "type_chirho": event_chirho.event_type_chirho,
                                "data_chirho": event_chirho.data_chirho
                            });
                            
                            match serde_json::to_string(&formatted_event_chirho) {
                                Ok(json_chirho) => {
                                    eprintln!("[WebSocket] Sending event to church_id {}: {}", church_id_chirho, json_chirho);
                                    std::io::stderr().flush().unwrap();
                                    if let Err(e_chirho) = socket_chirho.send(Message::Text(json_chirho.into())).await {
                                        eprintln!("[WebSocket] Failed to send event to church_id {}: {:?}", church_id_chirho, e_chirho);
                                        std::io::stderr().flush().unwrap();
                                        break;
                                    }
                                }
                                Err(e_chirho) => {
                                    eprintln!("[WebSocket] Failed to serialize event for church_id {}: {:?}", church_id_chirho, e_chirho);
                                    std::io::stderr().flush().unwrap();
                                }
                            }
                        }
                    }
                    Err(e_chirho) => {
                        eprintln!("[WebSocket] Error receiving broadcast message for church_id {}: {:?}", church_id_chirho, e_chirho);
                        std::io::stderr().flush().unwrap();
                        break;
                    }
                }
            }
            result = socket_chirho.recv() => {
                match result {
                    Some(Ok(msg_chirho)) => {
                        eprintln!("[WebSocket] Received message from church_id {}: {:?}", church_id_chirho, msg_chirho);
                        std::io::stderr().flush().unwrap();
                    }
                    Some(Err(e_chirho)) => {
                        eprintln!("[WebSocket] Error receiving message from church_id {}: {:?}", church_id_chirho, e_chirho);
                        std::io::stderr().flush().unwrap();
                        break;
                    }
                    None => {
                        eprintln!("[WebSocket] Client disconnected for church_id: {}", church_id_chirho);
                        std::io::stderr().flush().unwrap();
                        break;
                    }
                }
            }
        }
    }
    eprintln!("[WebSocket] Connection closed for church_id: {}", church_id_chirho);
    std::io::stderr().flush().unwrap();
}

#[derive(RustEmbed, Clone)]
#[folder = "assets_chirho/"]
struct AssetsChirho;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Force immediate output to verify logging is working
    println!("Server starting...");
    std::io::stdout().flush().unwrap();

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

    // Create broadcast channel for WebSocket events
    let (tx_chirho, _) = broadcast::channel(100);
    let state_chirho = Arc::new(AppStateChirho { tx: tx_chirho });

    let serve_assets_chirho = ServeEmbed::<AssetsChirho>::with_parameters(
        Some("/index.html".to_owned()),
        FallbackBehavior::NotFound,
        Some("index.html".to_owned()),
    );


    // Create router with combined state type
    let app_chirho = Router::new()
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
        .route("/api_chirho/admin_chirho/auth_chirho/logout_chirho", delete(logout_chirho))
        .route("/api_chirho/admin_chirho/pdf_files_chirho", post(upload_pdf_file_chirho))
        .route("/api_chirho/admin_chirho/pdf_files_chirho", get(get_all_pdf_files_chirho))
        .route("/api_chirho/admin_chirho/pdf_files_chirho/{pdf_file_id_chirho}", delete(delete_pdf_file_chirho))
        .route("/api_chirho/public_chirho/church_chirho/{church_token_chirho}", get(get_church_by_token_chirho))
        .route("/api_chirho/public_chirho/church_chirho/{church_token_chirho}/assign_to_schedule_chirho/{schedule_id_chirho}", post(create_signup_chirho))
        .route("/api_chirho/public_chirho/church_chirho/{church_token_chirho}/schedule_chirho/{schedule_id_chirho}/signups_chirho", get(get_schedule_signups_chirho))
        .route("/api_chirho/public_chirho/church_chirho/{church_token_chirho}/signup_chirho/{signup_id_chirho}", delete(delete_signup_chirho))
        .route("/api_chirho/public_chirho/schedules_chirho/upcoming_chirho", get(get_upcoming_schedules_chirho))
        .route("/api_chirho/public_chirho/pdf_files_chirho/{pdf_file_id_chirho}", get(serve_pdf_file_chirho))
        .route("/ws_chirho/{church_id_chirho}", get(websocket_handler_chirho))
        .fallback_service(serve_assets_chirho)
        .layer(cors_chirho)
        .with_state((pool_chirho, state_chirho));

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default();

    let config_chirho = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs_chirho")
            .join("server_chirho.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs_chirho")
            .join("server_chirho.key"),
    )
        .await
        .unwrap();
    // Start server
    let axum_string_port_chirho = std::env::var("AXUM_PORT_CHIRHO")
        .unwrap_or_else(|_| "3000".into());

    let addr_chirho = SocketAddr::from(([127, 0, 0, 1], axum_string_port_chirho.parse::<u16>().unwrap()));
    //let listener_chirho = TcpListener::bind(addr_chirho).await.unwrap();

    #[cfg(target_os = "openbsd")]
    {
        println!("JESUS CHRIST IS LORD");
    }

    println!("HALLELUJAH Server listening on {}", addr_chirho);
    std::io::stdout().flush().unwrap();
    axum_server::bind_rustls(addr_chirho, config_chirho)
        .serve(app_chirho.into_make_service())
        .await
        .unwrap();
    //axum::serve(listener_chirho, app_chirho).await.unwrap();
}
