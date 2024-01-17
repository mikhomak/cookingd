use async_graphql::SimpleObject;
use chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize)]
pub struct Tag {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
