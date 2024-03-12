use color_eyre::eyre::{eyre, Result};
use poem::{listener::TcpListener, Route};
use poem_openapi::OpenApiService;
use tracing::error;

use crate::api::routes::Api;

mod api;
mod git;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logger
    tracing_subscriber::fmt::init();

    // log startup event
    tracing::info!("Startup!");

    // install error handling
    color_eyre::install()?;

    // log eyre installation
    tracing::debug!("Eyre installed");

    // base path for repositories
    let base_path = "/path/to/base/repositories";

    // initialize API
    let api_service: OpenApiService<Api, ()> =
        OpenApiService::new(Api::new(base_path), "Hello World", "1.0")
            .server("http://localhost:3000/api");

    // initialize redoc and swagger
    let redoc = api_service.redoc();
    let swagger_ui = api_service.swagger_ui();

    // define application routes
    let app: Route = Route::new()
        .nest("/api", api_service)
        .nest("/redoc", redoc)
        .nest("/", swagger_ui);

    // run the server
    if let Err(err) = poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
    {
        error!("Poem Server Error: {}", err);
        return Err(eyre!("Poem Server Error: {}", err));
    }

    Ok(())
}
