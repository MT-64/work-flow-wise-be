#![allow(warnings)]
use crate::prisma::user;
use crate::prisma::Role;
use crate::swagger::ApiDoc;
use axum::response::{IntoResponse, Response};
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::dotenv;
use env::{check_env, hostname, https, port, setup_cors, ssl_cert_key};
use error::ErrorResponse;
use helpers::id::generate_id;
use objectives::service::ObjectiveService;
use prisma::PrismaClient;
use routes::routes;
use state::AppState;
use std::{
    net::SocketAddr,
    sync::{Arc, MutexGuard},
};
use tokio::net::TcpListener;
use tracing::info;
use tracing::Level;
use users::service::UserService;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::RoomState;
use axum::extract::State;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::broadcast;

mod aws;
mod comment;
mod department;
mod env;
mod error;
mod extractors;
mod file;
mod file_version;
mod folder;
mod helpers;
mod impls;
mod key_result;
mod notification;
mod objectives;
mod organize;
mod periods;
mod prisma;
mod response;
mod routes;
mod state;
mod swagger;
mod tag;
mod users;
mod validation;

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

    let mut routes = routes()
        .route("/api/v1/chat", get(handler))
        .layer(setup_cors())
        .with_state(state.clone());

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

        println!("Server started at https://{hostname}:{port}/docs");

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

async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut username = String::new();
    let mut channel = String::new();
    let mut tx = None::<broadcast::Sender<String>>;

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(name) = msg {
            #[derive(Deserialize)]
            struct Connect {
                username: String,
                channel: String,
            }

            let connect: Connect = match serde_json::from_str(&name) {
                ////// Save connection state with username ?, roome ?
                Ok(connect) => connect,
                Err(err) => {
                    println!("{}", &name);
                    println!("{}", err);
                    let _ = sender
                        .send(Message::from("Failed to connect to room!"))
                        .await;
                    break;
                }
            };

            {
                let mut rooms = state.rooms.lock().unwrap();
                channel = connect.channel.clone();

                let room = rooms.entry(connect.channel).or_insert_with(RoomState::new);
                tx = Some(room.tx.clone());

                if !room.users.lock().unwrap().contains(&connect.username) {
                    room.users
                        .lock()
                        .unwrap()
                        .insert(connect.username.to_owned());
                    username = connect.username.clone();
                }
            }

            if tx.is_some() && !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    let tx = tx.unwrap();
    let mut rx = tx.subscribe();

    let joined = format!("{} joined the chat!", username);
    let _ = tx.send(joined);

    let mut recv_messages = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut send_messages = {
        let tx = tx.clone();
        let name = username.clone();
        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                //// continue handle to save who send message ?
                let _ = tx.send(format!("{}: {}", name, text));
            }
        })
    };

    tokio::select! {
        _ = (&mut send_messages) => recv_messages.abort(),
        _ = (&mut recv_messages) => send_messages.abort(),
    };

    let left = format!("{} left the chat!", username);
    let _ = tx.send(left);
    let mut rooms = state.rooms.lock().unwrap();
    rooms
        .get_mut(&channel)
        .unwrap()
        .users
        .lock()
        .unwrap()
        .remove(&username);

    if rooms.get_mut(&channel).unwrap().users.lock().unwrap().len() == 0 {
        rooms.remove(&channel);
    }
}

async fn get_rooms(State(AppState { rooms, .. }): State<AppState>) -> String {
    let rooms = rooms.lock().unwrap();
    let vec = rooms.keys().into_iter().collect::<Vec<&String>>();
    match vec.len() {
        0 => json!({
            "status": "No rooms found yet!",
            "rooms": []
        })
        .to_string(),
        _ => json!({
            "status": "Success!",
            "rooms": vec
        })
        .to_string(),
    }
}
