use std::{env, fs::File, io::prelude::*, path::PathBuf, process};

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logger
    setup_tracing();
    info!("Startup!");

    // install error handling
    color_eyre::install()?;
    debug!("Eyre installed");

    // base path for repositories
    let base_path = "E:/RepoTests/Repos";

    let api_name = "Git";

    // Initialize API service
    let api_service = OpenApiService::new(Api::new(base_path), api_name, "1.0").server("/api");

    // check if --static-docs flag is provided
    if let Some(arg) = env::args_os().nth(1) {
        if arg == "--static-docs" {
            // Generate the Swagger UI and Redoc HTML content
            let swagger_ui_html_content = api_service.swagger_ui_html();
            let redoc_html_content = api_service.redoc_html();

            // Define the file paths for saving the HTML content
            let swagger_ui_file_path = PathBuf::from("swagger_ui.html");
            let redoc_file_path = PathBuf::from("redoc.html");

            // Create a new file and write the Swagger UI HTML content to it
            let mut swagger_file = File::create(&swagger_ui_file_path)?;
            swagger_file.write_all(swagger_ui_html_content.as_bytes())?;

            // Create a new file and write the Redoc HTML content to it
            let mut redoc_file = File::create(&redoc_file_path)?;
            redoc_file.write_all(redoc_html_content.as_bytes())?;

            // Generate the HTML content
            let html_content = format!(
                r#"<html><head><title>{}</title><style>body{{font-family:Arial,sans-serif;display:flex;flex-direction:column;align-items:center;justify-content:center;height:100vh;margin:0;}}.button{{padding:20px 40px;font-size:18px;margin:10px;border:none;cursor:pointer;border-radius:8px;background-color:#007bff;color:white;text-align:center;text-decoration:none;transition:background-color 0.3s;}}.button:hover{{background-color:#0056b3;}}</style></head><body><h1>Welcome to {} API Documentation</h1><a href="redoc.html" class="button">Redoc</a><a href="swagger_ui.html" class="button">Swagger UI</a></body></html>"#,
                api_name, api_name
            );
            let mut file = File::create("index.html")?;
            file.write_all(html_content.as_bytes())?;

            // Log the generation of Swagger UI and Redoc HTML files
            info!("Swagger UI & Redoc HTML files generated");

            // Exit the program
            return Ok(());
        }
    }

    // Define application routes
    let app: Route = Route::new()
        .nest("/redoc", api_service.redoc())
        .nest("/docs", api_service.swagger_ui())
        .nest("/api", api_service)
        .nest(
            "/",
            StaticFilesEndpoint::new("./web/")
                .show_files_listing()
                .index_file("index.html"),
        );

    // Run the server
    if let Err(err) = poem::Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app.with(Cors::new()))
        .await
    {
        error!("Poem Server Error: {}", err);
        process::exit(1);
    }

    Ok(())
}
