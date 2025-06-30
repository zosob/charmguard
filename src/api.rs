use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
use std::sync::{Arc, Mutex};
mod timer;

//-----Shared state-----
#[derive(Default)]
pub struct AppState{
    pub active: Mutex<bool>,
    pub minutes_remaining: Mutex<u32>,
}

#[derive(Deserialize)]
struct StartPayload{
    minutes: u32,
}

#[post("/start")]
async fn start(
    payload: web::Json<StartPayload>,
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    {
        let mut active = state.active.lock().unwrap();
        if *active {
            return HttpResponse::BadRequest().body("Session already running");
        }
        *active = true;
    }
    
    let mins = payload.minutes;
    let state_clone = state.clone();

    //Spawn timer in background Tokio task
    tokio::spawn({
        let state_clone = state.clone();
        async move{
            for min in (0..mins).rev(){
                *state_clone.minutes_remaining.lock().unwrap() = min;
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
            *state_clone.active.lock().unwrap() = false;
        }
    });
    HttpResponse::Ok().body(format!("Started {}-minute session", mins))
}

#[post("/stop")]
async fn stop(state: web::Data<Arc<AppState>>) -> impl Responder {
    *state.active.lock().unwrap() = false;
    HttpResponse::Ok().body("Session stopped")
}

#[get("/status")]
async fn status(state: web::Data<Arc<AppState>>) -> impl Responder {
    let active = *state.active.lock().unwrap();
    let mins = *state.minutes_remaining.lock().unwrap();
    HttpResponse::Ok().json(json!({
        "state": if active { "active" } else {"idle" },
        "minutes_remaining": mins
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared = Arc::new(AppState::default());
    println!("CharmGuard API listening on http://localhost:9090");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared.clone()))
            .service(start)
            .service(stop)
            .service(status)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}