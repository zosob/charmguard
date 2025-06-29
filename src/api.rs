use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
mod timer;

//-----Shared state-----
#[derive(Default)]
struct AppState{
    active: Mutex<bool>,
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
    tokio::spawn(async move{
        timer::start(mins);
        *state_clone.active.lock().unwrap() = false;
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
    let msg = if active { "active" } else { "idle" };
    HttpResponse::Ok().body(msg)
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