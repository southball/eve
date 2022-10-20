use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{header, StatusCode},
    Extension,
};
use sqlx::PgPool;

pub struct AccountId(pub i64);

#[async_trait]
impl<B> FromRequest<B> for AccountId
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(_pg_pool): Extension<PgPool> = Extension::<PgPool>::from_request(req)
            .await
            .map_err(|_err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch database.",
            )
        })?;

        if let Some(_authorization) = req.headers().get(header::AUTHORIZATION) {
            // TODO: implement extraction from token
            Ok(AccountId(0))
        } else if let Some(_cookies) = req.headers().get(header::COOKIE) {
            // TODO: implement extraction from cookie
            Ok(AccountId(0))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Not authorized."))
        }
    }
}
