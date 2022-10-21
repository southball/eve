use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};

fn get_session_cookie_key() -> String {
    std::env::var("EVE_SERVER_SESSION_ID_COOKIE_KEY")
        .unwrap_or_else(|_| "EVE_SESSION_ID".to_string())
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub account_id: Option<i32>,
}

#[allow(clippy::derivable_impls)]
impl Default for SessionData {
    fn default() -> Self {
        SessionData { account_id: None }
    }
}

pub struct Session {
    id: Option<String>,
    data: SessionData,
    pg_pool: PgPool,
    cookies: Cookies,
}

impl Session {
    pub fn get(&self) -> &SessionData {
        &self.data
    }

    pub async fn set(&mut self, data: SessionData) -> anyhow::Result<()> {
        let id = self
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        self.id = Some(id.clone());

        self.data = data;
        let serialized_session = serde_json::to_string(&self.data).unwrap();

        sqlx::query!(
            "
                INSERT INTO cookie_session (id, content, expiry)
                VALUES ($1, $2, CURRENT_TIMESTAMP + INTERVAL '1 day')
                ON CONFLICT (id)
                DO UPDATE SET content = $2, expiry = CURRENT_TIMESTAMP + INTERVAL '1 day'
            ",
            &id,
            &serialized_session
        )
        .execute(&self.pg_pool)
        .await
        .map_err(|_err| anyhow::anyhow!("Failed to save session."))?;

        self.cookies.add(Cookie::new(get_session_cookie_key(), id));

        Ok(())
    }
}

#[async_trait]
impl<B> FromRequest<B> for Session
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pg_pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(|_err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Session: failed to fetch database.",
                )
            })?;

        let cookies = Cookies::from_request(req).await?;

        let (session_id, session_data) = match cookies.get(&get_session_cookie_key()) {
            Some(key) => {
                let row = sqlx::query!(
                    "
                    SELECT content, expiry FROM cookie_session
                    WHERE id = $1 AND expiry > CURRENT_TIMESTAMP
                ",
                    key.value()
                )
                .fetch_optional(&pg_pool)
                .await
                .unwrap();

                if row.is_some() {
                    sqlx::query!(
                        "
                            UPDATE cookie_session
                            SET expiry = CURRENT_TIMESTAMP + INTERVAL '1 day'
                            WHERE id = $1 AND expiry < CURRENT_TIMESTAMP + INTERVAL '12 hours'
                        ",
                        key.value()
                    )
                    .execute(&pg_pool)
                    .await
                    .unwrap();
                }

                row.map(|record| record.content)
                    .and_then(|content| serde_json::from_str(&content).ok())
                    .map(|session_data| (Some(key.value().to_owned()), session_data))
            }
            None => None,
        }
        .unwrap_or_else(|| (None, SessionData::default()));

        Ok(Session {
            id: session_id,
            data: session_data,
            pg_pool,
            cookies,
        })
    }
}
