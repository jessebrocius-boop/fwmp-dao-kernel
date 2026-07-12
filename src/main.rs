use std::net::SocketAddr;

mod interfaces;

#[tokio::main]
async fn main() {
    // Build our application with the ingress route
    let app = interfaces::api::router();

    // Run it on 0.0.0.0:8080 so it's container-friendly
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Ingress gatekeeper listening on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");
}
