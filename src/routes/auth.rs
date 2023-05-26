use crate::{config::AuthConfig, AppState};
use anyhow::{Context, Result};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata, CoreResponseType},
    reqwest::async_http_client,
    AccessTokenHash, AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse,
};

async fn login(
    State(state): State<AppState>,
    mut session: WritableSession,
) -> Result<impl IntoResponse, StatusCode> {
    if state.oidc_client.is_none() {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication is not configured for this server, unable to log in.",
        )
            .into_response());
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
        .map_err(|e| {
            tracing::error!("Failed to insert PKCE verifier into session: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    session.insert("nonce", nonce).map_err(|e| {
        tracing::error!("Failed to insert nonce into session: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(axum::response::Redirect::to(auth_url.as_str()).into_response())
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
    session_state: String,
}

async fn callback(
    Query(params): Query<AuthRequest>,
    State(state): State<AppState>,
    mut session: WritableSession,
) -> Result<impl IntoResponse, StatusCode> {
    if state.oidc_client.is_none() {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication is not configured for this server, unable to log in.",
        )
            .into_response());
    }
    let client = state.oidc_client.unwrap();

    let pkce_verifier = session
        .get::<PkceCodeVerifier>("pkce_verifier")
        .context("Failed to get PKCE verifier from session")
        .map_err(|e| {
            tracing::error!("Failed to get PKCE verifier from session: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let nonce = session
        .get::<Nonce>("nonce")
        .context("Failed to get nonce from session")
        .map_err(|e| {
            tracing::error!("Failed to get nonce from session: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    session.remove("pkce_verifier");
    session.remove("nonce");

    let token_response = client
        .exchange_code(AuthorizationCode::new(params.code))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get token response from session: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Extract the ID token claims after verifying its authenticity and nonce.
    let id_token = token_response.id_token().unwrap();

    let claims = id_token
        .claims(&client.id_token_verifier(), &nonce)
        .unwrap();

    // Verify the access token hash to ensure that the access token hasn't been substituted for
    // another user's.
    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            &id_token.signing_alg().unwrap(),
        )
        .unwrap();
        if actual_access_token_hash != *expected_access_token_hash {
            panic!()
        }
    }
    //insert username from token claim into session
    //if is exec, redirect to /app/panel, else redirect to /app/unauth

    Ok((
        StatusCode::OK,
        format!(
            "Redirected! your name is {:?}",
            claims.preferred_username().unwrap()
        ),
    )
        .into_response())
}

#[tracing::instrument]
pub async fn oidc_client(config: AuthConfig) -> anyhow::Result<CoreClient> {
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
