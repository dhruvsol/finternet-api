use error_stack::ResultExt;
use tokio::net::TcpListener;

use crate::error::{ConfigurationError, SResult};
use crate::logging::prelude::*;

mod token_managers;
mod users;

pub fn router<S: Send + Sync + Clone + 'static>() -> SResult<axum::Router<S>, ConfigurationError> {
    let router = axum::Router::new()
        .nest("/v1/users", users::router()?)
        .nest("/v1/token_managers", token_managers::router()?);

    Ok(router)
}

pub async fn start_server(
    router: axum::Router,
    listener: TcpListener,
) -> SResult<(), ConfigurationError> {
    info!("Starting server");
    info!(
        "Listening on: {}",
        listener
            .local_addr()
            .change_context(ConfigurationError::LocalAddressError)?
    );
    axum::serve(listener, router)
        .await
        .change_context(ConfigurationError::ServerStartError)
}
