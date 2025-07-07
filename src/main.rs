use axum::{ 
    routing::post, Router
};

mod globals;
mod controllers;
use controllers::execution_controller::submit_code;
use controllers::execution_controller::submit_batch;
mod models;
use models::submission::SubmissionJob;
mod services;
use services::worker::worker_loop;
mod utils;

use std::collections::VecDeque;
use std::sync::Mutex;
use lazy_static::lazy_static;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/submissions", post(submit_code))
        .route("/submissions/batch", post(submit_batch));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    let num_workers = num_cpus::get();

    for _ in 0..num_workers {
        tokio::spawn(worker_loop());
    }

    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

lazy_static! {
    static ref SUBMISSION_QUEUE: Mutex<VecDeque<SubmissionJob>> = Mutex::new(VecDeque::new());
}

