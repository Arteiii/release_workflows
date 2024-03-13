use poem::{
    get, handler,
    listener::TcpListener,
    web::{Path, Redirect},
    IntoResponse, Route, Server,
};

#[handler]
pub async fn redirect(Path(name): Path<String>) -> Redirect {
    let msg = format!("redirecting to: {}", name);
    tracing::debug!(msg);

    // TODO: Add a actualyl working git server
    // supporting push pull co
    // (idfk how)
    Redirect::moved_permanent("/test123/again")
}
