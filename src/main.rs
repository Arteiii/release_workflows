mod api;
// mod git;
// mod util;

use color_eyre::eyre::Result;
use poem::{listener::TcpListener, Route};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(api::routes::Api, "Hello World", "1.0")
        .server("http://localhost:3000/api");

    let redoc = api_service.redoc();
    let swagger_ui = api_service.swagger_ui();

    api_service.redoc_html();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs/redoc", redoc)
        .nest("/docs/swagger", swagger_ui);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
