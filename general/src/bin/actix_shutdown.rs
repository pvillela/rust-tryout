use actix_web::{web, App, HttpResponse, HttpServer};
use std::{process, time::Duration};
use tokio;

#[actix_web::main]
async fn main() {
    let h = tokio::spawn(async {
        let wait_secs = 10;
        println!("Going to let server run for {wait_secs} secs and then will terminate it.");
        tokio::time::sleep(Duration::from_secs(wait_secs)).await;
        let my_pid = process::id();
        let mut kill = process::Command::new("kill")
            .args(["-s", "TERM", &my_pid.to_string()])
            .spawn()
            .unwrap();
        kill.wait().unwrap();
    });

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
        .unwrap();

    h.await.unwrap();
    println!("Actix terminated gradefully.")
}
