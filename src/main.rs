use std::{env, fs::File, io::prelude::*, path::PathBuf, process};

use color_eyre::eyre::{eyre, Result};
use handlebars::Handlebars;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, EndpointExt, Route};
use poem_openapi::OpenApiService;
use pulldown_cmark::{html, Options, Parser};
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
    // Initialize logger
    setup_tracing();
    info!("Startup!");

    // Install error handling
    color_eyre::install()?;
    debug!("Eyre installed");

    // Base path for repositories
    let base_path = "E:/RepoTests/Repos";

    let api_name = "Git";

    // Initialize API service
    let api_service = OpenApiService::new(Api::new(base_path), api_name, "1.0").server("/api");

    // Check if --static-docs flag is provided
    if let Some(arg) = env::args_os().nth(1) {
        if arg == "--static-docs" {
            generate_static_docs(&api_service, api_name)?;
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
            StaticFilesEndpoint::new("src/web/")
                .show_files_listing()
                .index_file("index.html"),
        );

    // Run the server
    if let Err(err) = poem::Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
    {
        error!("Poem Server Error: {}", err);
        process::exit(1);
    }

    Ok(())
}

/// Generate static documentation files
fn generate_static_docs(api_service: &OpenApiService<Api, ()>, api_name: &str) -> Result<()> {
    // Generate the Swagger UI and Redoc HTML content
    let swagger_ui_html_content = api_service.swagger_ui_html();
    let redoc_html_content = api_service.redoc_html();

    // Write the Swagger UI HTML content to file
    let mut swagger_file = File::create("swagger_ui.html")?;
    swagger_file.write_all(swagger_ui_html_content.as_bytes())?;

    // Write the Redoc HTML content to file
    let mut redoc_file = File::create("redoc.html")?;
    redoc_file.write_all(redoc_html_content.as_bytes())?;

    // Convert Installation instructions Markdown to HTML
    let installation_md_content = include_str!("../Installation.md");
    let installation_html_content = markdown_to_html_with_line_breaks(installation_md_content)?;
    // Write the Installation HTML content to file
    let mut installation_file = File::create("installation.html")?;
    installation_file.write_all(installation_html_content.as_bytes())?;

    // Convert License Markdown to HTML
    let license_md_content = include_str!("../LICENSE.md");
    let license_html_content = markdown_to_html_with_line_breaks(license_md_content)?;
    // Write the License HTML content to file
    let mut license_file = File::create("license.html")?;
    license_file.write_all(license_html_content.as_bytes())?;

    // Generate the HTML content from the template
    let html_template = include_str!("web/index_template.html");
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("index_template", html_template)?;
    let mut data = std::collections::BTreeMap::new();
    data.insert("api_name", api_name);
    let html_content = handlebars.render("index_template", &data)?;

    // Write the rendered HTML content to the index.html file
    let mut file = File::create("index.html")?;
    file.write_all(html_content.as_bytes())?;

    info!("Static documentation files generated");

    Ok(())
}

/// Convert Markdown to HTML
/// Convert Markdown to HTML with preserved line breaks
fn markdown_to_html_with_line_breaks(markdown: &str) -> Result<String> {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output.replace("\n", "<br>")) // replace newline characters with <br> tags
}
