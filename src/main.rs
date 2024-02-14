#![allow(warnings)]
use crate::prisma::user;
use crate::prisma::Role;
use crate::swagger::ApiDoc;
use axum::response::Response;
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::dotenv;
use env::{check_env, hostname, https, port, setup_cors, ssl_cert_key};
use error::ErrorResponse;
use helpers::id::generate_id;
use objectives::service::ObjectiveService;
use prisma::PrismaClient;
use routes::routes;
use state::AppState;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::info;
use tracing::Level;
use users::service::UserService;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod department;
mod env;
mod error;
mod extractors;
mod helpers;
mod objectives;
mod prisma;
mod response;
mod routes;
mod state;
mod swagger;
mod users;

pub type WebResult = std::result::Result<Response, ErrorResponse>;
#[tokio::main]
async fn main() {
    dotenv().ok();
    //   check_env();
    tracing_subscriber::fmt().init();

    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Cannot connect to Postgres");

    let client = Arc::new(client);

    let state = AppState::new(client).await;

    let mut routes = routes().layer(setup_cors()).with_state(state);

    let hostname = hostname();
    let port = port();

    routes = routes.merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    if https() {
        let tls_config = {
            let (cert_path, key_path) = ssl_cert_key();
            RustlsConfig::from_pem_file(cert_path, key_path)
                .await
                .expect("Cannot find certifications to enable https")
        };

        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        println!("Server started at https://{hostname}:{port}");

        axum_server::bind_rustls(addr, tls_config)
            .serve(routes.into_make_service())
            .await
            .expect("Server crashed")
    } else {
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

        println!("Server started at http://{hostname}:{port}");

        axum::serve(listener, routes).await.expect("Server crashed");
    }
}
