use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{errors::DomainError, user::User};
use crate::usecase::ports::UserRepository;

#[derive(Clone)]
pub struct SqlxUserRepo {
    pool: PgPool,
}

impl SqlxUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    email: String,
    #[sqlx(rename = "active")]
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        User {
            id: r.id,
            username: r.username,
            email: r.email,
            active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepo {
    async fn create(&self, username: String, email: String, active: bool) -> Result<User, DomainError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            INSERT INTO users (username, email, active)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, active, created_at, updated_at
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(active)
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx_err)?;

        Ok(row.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, username, email, active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_err)?;

        Ok(row.map(Into::into))
    }

    async fn list(&self, active: Option<bool>, limit: i64, offset: i64) -> Result<Vec<User>, DomainError> {
        let rows = match active {
            Some(a) => {
                sqlx::query_as::<_, UserRow>(
                    r#"
                    SELECT id, username, email, active, created_at, updated_at
                    FROM users
                    WHERE active = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                )
                .bind(a)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx_err)?
            }
            None => {
                sqlx::query_as::<_, UserRow>(
                    r#"
                    SELECT id, username, email, active, created_at, updated_at
                    FROM users
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                    "#,
                )
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx_err)?
            }
        };

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update(
        &self,
        id: Uuid,
        username: Option<String>,
        email: Option<String>,
        active: Option<bool>,
    ) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            UPDATE users
            SET
              username = COALESCE($2, username),
              email    = COALESCE($3, email),
              active   = COALESCE($4, active),
              updated_at = now()
            WHERE id = $1
            RETURNING id, username, email, active, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(username)
        .bind(email)
        .bind(active)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_err)?;

        Ok(row.map(Into::into))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
        let res = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_sqlx_err)?;

        Ok(res.rows_affected() > 0)
    }
}

fn map_sqlx_err(err: sqlx::Error) -> DomainError {
    if let sqlx::Error::Database(db_err) = &err {
        if let Some(code) = db_err.code() {
            if code == "23505" {
                return DomainError::Conflict("duplicate value (likely email)".into());
            }
        }
    }
    DomainError::Unexpected(err.to_string())
}
