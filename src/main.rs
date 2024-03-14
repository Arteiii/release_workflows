use std::{env, fs::File, io::prelude::*, process};

use color_eyre::eyre::Result;
use handlebars::Handlebars;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route};
use poem_openapi::OpenApiService;
use pulldown_cmark::{html, Options, Parser};
use tracing::{debug, error, info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

use crate::api::routes::Api;

mod api;
mod build;
mod git;
mod util;

// TODO: remove its jsut for dev
const BASE_PATH: &str = "E:/RepoTests/Repos";
const API_NAME: &str = "Git";
const SWAGGER_UI_FILE: &str = "swagger_ui.html";
const REDOC_FILE: &str = "redoc.html";
const INSTALLATION_FILE: &str = "installation.html";
const LICENSE_FILE: &str = "license.html";
const README_FILE: &str = "README.md";
const INDEX_FILE: &str = "index.html";

fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();
    info!("Startup!");

    color_eyre::install()?;
    debug!("Eyre installed");

    let api_service = OpenApiService::new(Api::new(BASE_PATH), API_NAME, "1.0").server("/api");

    if let Some(arg) = env::args_os().nth(1) {
        if arg == "--static-docs" {
            generate_static_docs(&api_service)?;
            return Ok(());
        }
    }

    let app: Route = Route::new()
        .nest("/redoc", api_service.redoc())
        .nest("/docs", api_service.swagger_ui())
        .nest("/api", api_service)
        .nest(
            "/",
            StaticFilesEndpoint::new("src/web/")
                .show_files_listing()
                .index_file(INDEX_FILE),
        );

    if let Err(err) = poem::Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
    {
        error!("Poem Server Error: {}", err);
        process::exit(1);
    }

    Ok(())
}

fn generate_static_docs(api_service: &OpenApiService<Api, ()>) -> Result<()> {
    let swagger_ui_html_content = api_service.swagger_ui_html();
    let redoc_html_content = api_service.redoc_html();

    write_to_file(SWAGGER_UI_FILE, &swagger_ui_html_content)?;
    write_to_file(REDOC_FILE, &redoc_html_content)?;

    let installation_md_content = include_str!("../Installation.md");
    let installation_html_content = markdown_to_html_with_line_breaks(installation_md_content)?;
    write_to_file(INSTALLATION_FILE, &installation_html_content)?;

    let license_md_content = include_str!("../LICENSE.md");
    let license_html_content = markdown_to_html_with_line_breaks(license_md_content)?;
    write_to_file(LICENSE_FILE, &license_html_content)?;

    let readme_content = include_str!("../README.md");
    let readme_html_content = markdown_to_html_with_line_breaks(readme_content)?;

    let html_template = include_str!("web/index_template.html");
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("index_template", html_template)?;
    let mut data = std::collections::BTreeMap::new();
    data.insert("api_name", API_NAME);
    data.insert("readme", &readme_html_content);
    let html_content = handlebars.render("index_template", &data)?;

    write_to_file(INDEX_FILE, &html_content)?;

    info!("Static documentation files generated");

    Ok(())
}

fn write_to_file(file_name: &str, content: &str) -> Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn markdown_to_html_with_line_breaks(markdown: &str) -> Result<String> {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output.replace("\n", "<br>"))
}
