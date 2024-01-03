use axum::{
    routing::{get, post},
    Form, Router,
};
use axum_login::AuthSession;

use crate::{
    errors::ApplicationError,
    models::{Backend, Credentials},
};

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
}

async fn login(
    mut auth_session: AuthSession<Backend>,
    Form(creds): Form<Credentials>,
) -> Result<(), ApplicationError> {
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ApplicationError::NotFound),
        Err(err) => return Err(err.into()),
    };

    auth_session.login(&user).await?;

    Ok(())
}

async fn logout(mut auth_session: AuthSession<Backend>) -> Result<(), ApplicationError> {
    auth_session.logout().await?;
    Ok(())
}
