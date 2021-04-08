#[macro_use]
extern crate log;

use env_logger;

mod error;
mod models;
mod u_client;

use crate::models::AppState;
use crate::u_client::UnifiClient;
use actix_web::*;
use models::{Bandwidth, DndState};
use std::env;
use std::sync::Arc;
use actix_cors::Cors;

#[get("/state")]
async fn get_state(state: web::Data<AppState>) -> HttpResponse {
    match state.client.get_state().await {
        Ok(s) => match s {
            Some(rate) => {
                if rate.max_up == state.on.up && rate.max_down == state.on.down {
                    HttpResponse::Ok().json(DndState::new(true))
                } else {
                    HttpResponse::Ok().json(DndState::new(false))
                }
            }
            None => HttpResponse::InternalServerError().body("No rate found"),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

#[post("/state")]
async fn set_state(state: web::Data<AppState>, rq: web::Json<DndState>) -> HttpResponse {
    let res = match rq.state {
        true => state.client.set_state(state.on.down, state.on.up),
        false => state.client.set_state(state.off.down, state.off.up),
    }
    .await;
    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
    .body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let off_down = env::var("UNIFI_OFF_DOWN")
        .unwrap_or_else(|_| "-1".to_string())
        .parse()
        .expect("unable to convert UNIFI_OFF_DOWN to int");
    let off_up = env::var("UNIFI_OFF_UP")
        .unwrap_or_else(|_| "-1".to_string())
        .parse()
        .expect("unable to convert UNIFI_OFF_UP to int");
    let off_bandwidth: Bandwidth = Bandwidth {
        down: off_down,
        up: off_up,
    };

    let on_down = env::var("UNIFI_ON_DOWN")
        .unwrap_or_else(|_| "20000".to_string())
        .parse()
        .expect("unable to convert UNIFI_ON_DOWN to int");
    let on_up = env::var("UNIFI_ON_UP")
        .unwrap_or_else(|_| "-1".to_string())
        .parse()
        .expect("unable to convert UNIFI_ON_UP to int");
    let on_bandwidth: Bandwidth = Bandwidth {
        down: on_down,
        up: on_up,
    };

    let unifi_client: Arc<UnifiClient> = Arc::new(UnifiClient {
        url: env::var("UNIFI_URL").expect("UNIFI_URL should be provided"),
        user: env::var("UNIFI_USER").expect("UNIFI_USER should be provided"),
        password: env::var("UNIFI_PWD").expect("UNIFI_PWD should be provided"),
        site: env::var("UNIFI_SITE").unwrap_or_else(|_| "default".to_string()),
        group: env::var("UNIFI_GROUP").expect("UNIFI_GROUP should be provided"),
        client: reqwest::Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
    });

    let port = env::var("PORT").unwrap_or("8080".to_string());
    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST"]);

        App::new()
            .wrap(cors)
            .data(AppState {
                on: on_bandwidth,
                off: off_bandwidth,
                client: unifi_client.clone(),
            })
            .service(get_state)
            .service(set_state)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
