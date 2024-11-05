use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum BlockKind {
    Page,
    Text,
}

pub struct Block<P> {
    pub id: Uuid,
    pub kind: BlockKind,
    pub parent_id: Option<Uuid>,
    pub content: Vec<Uuid>,
    pub props: P,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
