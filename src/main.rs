use color_eyre::eyre::{eyre, Result};
use poem::middleware::Cors;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, EndpointExt, Route};
use poem_openapi::OpenApiService;
use tracing::{debug, error, info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

use crate::api::routes::Api;

mod api;
mod build;
mod git;

mod util;

fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logger
    setup_tracing();

    // log startup event
    info!("Startup!");

    // install error handling
    color_eyre::install()?;

    // log eyre installation
    debug!("Eyre installed");

    // base path for repositories
    let base_path = "E:/RepoTests/Repos";

    // initialize API
    let api_service: OpenApiService<Api, ()> =
        OpenApiService::new(Api::new(base_path), "Git", "1.0").server("/api");

    // initialize redoc and swagger
    let redoc = api_service.redoc();
    let swagger_ui = api_service.swagger_ui();

    // define application routes
    let app: Route = Route::new()
        .nest("/api", api_service)
        .nest("/redoc", redoc)
        .nest(
            "/",
            StaticFilesEndpoint::new("E:/RepoTests/Repos/")
                .show_files_listing()
                .index_file("index.html"),
        )
        .nest("/docs", swagger_ui);

    // run the server
    if let Err(err) = poem::Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app.with(Cors::new()))
        .await
    {
        error!("Poem Server Error: {}", err);
        return Err(eyre!("Poem Server Error: {}", err));
    }

    Ok(())
}
