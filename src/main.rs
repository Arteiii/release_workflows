use color_eyre::eyre::Result;
use poem::{listener::TcpListener, Route};
use poem_openapi::OpenApiService;
use tracing::{event, Level};

use crate::api::routes::Api;
// use git::manager::RepositoryManager;

mod api;
// mod git;
// mod util;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    event!(Level::INFO, "Startup!");

    color_eyre::install()?;

    event!(Level::DEBUG, "eyre installed");

    let api_service: OpenApiService<Api, ()> =
        OpenApiService::new(api::routes::Api, "Hello World", "1.0")
            .server("http://localhost:3000/api");

    let redoc = api_service.redoc();
    let swagger_ui = api_service.swagger_ui();

    let app: Route = Route::new()
        .nest("/api", api_service)
        .nest("/docs/redoc", redoc)
        .nest("/docs/swagger", swagger_ui);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
        .expect("TODO: panic message");

    Ok(())
}
