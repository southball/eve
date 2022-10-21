use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{header, StatusCode},
    Extension,
};
use sqlx::PgPool;

use crate::session::Session;

pub struct AccountId(pub i32);

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

        let session = Session::from_request(req).await?;

        if let Some(_authorization) = req.headers().get(header::AUTHORIZATION) {
            // TODO: implement extraction from token
            Ok(AccountId(0))
        } else if let Some(account_id) = session.get().account_id {
            // TODO: implement extraction from cookie
            Ok(AccountId(account_id))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Not authorized."))
        }
    }
}
