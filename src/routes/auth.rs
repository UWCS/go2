use std::ops::Deref;

use super::handle_error;
use crate::{config::AuthConfig, AppState};
use anyhow::{anyhow, Context};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Result},
    routing::get,
    Router,
};
use axum_sessions::extractors::WritableSession;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
};

#[tracing::instrument]
async fn login(
    State(state): State<AppState>,
    mut session: WritableSession,
) -> Result<impl IntoResponse> {
    if state.oidc_client.is_none() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication is not configured for this server, unable to log in.",
        )
            .into());
    }
    let client = state.oidc_client.unwrap();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, _csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // Set the desired scopes.
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session
        .insert("pkce_verifier", pkce_verifier)
        .context("Could not insert pkce verifier into session store")
        .map_err(handle_error)?;

    session
        .insert("nonce", nonce)
        .context("Could not insert nonce into session store")
        .map_err(handle_error)?;

    Ok(axum::response::Redirect::to(auth_url.as_str()).into_response())
}

#[derive(Debug, serde::Deserialize)]
#[allow(unused)]
struct AuthRequest {
    code: String,
    state: String,
    session_state: String,
}

#[tracing::instrument]
async fn callback(
    Query(params): Query<AuthRequest>,
    State(state): State<AppState>,
    mut session: WritableSession,
) -> axum::response::Result<impl IntoResponse> {
    // ensure that client exists
    if state.oidc_client.is_none() {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication is not configured for this server, unable to log in.",
        )
            .into_response());
    }
    let client = state.oidc_client.unwrap();

    //get the PKCE verifier and nonce
    let pkce_verifier = session
        .get::<PkceCodeVerifier>("pkce_verifier")
        .context("Failed to get PKCE verifier from session")
        .map_err(handle_error)?;

    let nonce = session
        .get::<Nonce>("nonce")
        .context("Failed to get nonce from session")
        .map_err(handle_error)?;

    session.remove("pkce_verifier");
    session.remove("nonce");

    //swap the code for a token
    let token_response = client
        .exchange_code(AuthorizationCode::new(params.code))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .context("Failed to get token response from OIDC provider")
        .map_err(handle_error)?;

    // Extract the ID token claims after verifying it
    let id_token = token_response
        .id_token()
        .context("Failed to extract ID token")
        .map_err(handle_error)?;

    //get claims from token
    let claims = id_token
        .claims(&client.id_token_verifier(), &nonce)
        .context("Failed to get token claims")
        .map_err(handle_error)?;

    // Verify the access token hash to ensure that the access token is good
    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            &id_token
                .signing_alg()
                .context("Failed to get token signing algorithm")
                .map_err(handle_error)?,
        )
        .context("Failed to get token hash")
        .map_err(handle_error)?;
        if actual_access_token_hash != *expected_access_token_hash {
            handle_error(anyhow!("Access token hash did not match"));
        }
    }
    let username = claims
        .preferred_username()
        .map(Deref::deref)
        .unwrap_or_else(|| {
            claims
                .nickname()
                .expect("Tried *real* hard but could not get the username from this OIDC provider")
                .get(None)
                .unwrap()
            //this crate is so unbelievably awkward
        });
    //insert username into session store
    session
        .insert("username", username)
        .context("Failed to insert username into session store")
        .map_err(handle_error)?;

    Ok(Redirect::to("/app/panel").into_response())
}

#[tracing::instrument]
pub async fn oidc_client(config: AuthConfig) -> anyhow::Result<CoreClient> {
    //get OIDC metadata from provider
    let keycloak_odic_info = CoreProviderMetadata::discover_async(
        IssuerUrl::new(config.oidc_url.clone())?,
        async_http_client,
    )
    .await
    .context(format!(
        "Could not get OIDC info from provider at {}",
        &config.oidc_url,
    ))?;

    Ok(CoreClient::from_provider_metadata(
        keycloak_odic_info,
        ClientId::new(config.client_id),
        Some(ClientSecret::new(config.client_secret)),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(format!(
        "{}/auth/callback",
        config.app_url
    ))?))
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
}
