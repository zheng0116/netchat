use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;

pub async fn auth_middleware(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    crate::verify_token(auth.token()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(next.run(request).await)
}
