use chrono::{DateTime, Utc};
use leptos::prelude::*;
use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Clone, Debug, Display, FromStr, Serialize, Deserialize)]
pub enum BlockKind {
    Page,
    Text,
}

#[derive(Clone, Debug, FromRow)]
pub struct SqlBlock {
    pub id: String,
    pub kind: String,
    pub parent_id: Option<String>,
    pub children: String,
    pub props: String,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub done: bool,
}

pub fn pool() -> anyhow::Result<SqlitePool> {
    use_context::<SqlitePool>().ok_or_else(|| anyhow::anyhow!("Pool missing."))
}

impl SqlBlock {
    pub async fn create(
        kind: BlockKind,
        parent_id: Option<Uuid>,
        children: Vec<Uuid>,
        props: String,
    ) -> anyhow::Result<()> {
        let id = Uuid::now_v7();
        let children = children
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let pool = pool()?;
        let query = sqlx::query(
            r#"
            INSERT INTO blocks (id, kind, parent_id, children, props) 
            VALUES             (?,  ?,    ?,         ?,        ?)
            "#,
        )
        .bind(id)
        .bind(kind.to_string())
        .bind(parent_id)
        .bind(children)
        .bind(props);
        query.execute(&pool).await?;
        Ok(())
    }

    pub async fn list_pages() -> anyhow::Result<Vec<SqlBlock>> {
        let pool = pool()?;
        let res = sqlx::query_as::<_, SqlBlock>(r#"SELECT * FROM blocks WHERE kind = 'page'"#)
            .fetch_all(&pool)
            .await?;

        Ok(res)
    }

    pub async fn list(parent_id: Uuid) -> anyhow::Result<Vec<SqlBlock>> {
        let pool = pool()?;
        let res = sqlx::query_as::<_, SqlBlock>(
            r#"
            WITH RECURSIVE
                child_blocks(id, kind, parent_id, children, props, start, end, done) AS (
                    SELECT id, kind, parent_id, children, props, start, end, done FROM blocks WHERE id = ?
                    UNION
                    SELECT b.id AS id, b.kind as kind, b.parent_id as parent_id, b.children as children, b.props as props, b.start as start, b.end AS end, b.done AS done FROM blocks AS b, child_blocks
                     WHERE b.parent_id = child_blocks.id AND (child_blocks.kind != 'page' OR child_blocks.id = ?)
                )
            SELECT * FROM child_blocks
        "#,
        )
        .bind(parent_id.to_string())
        .bind(parent_id.to_string())
        .fetch_all(&pool)
        .await?;

        tracing::info!(parent_id = ?parent_id, list_blocks = ?res);

        Ok(res)
    }
}
