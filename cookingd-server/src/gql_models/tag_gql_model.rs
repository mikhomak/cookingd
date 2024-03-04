use async_graphql::SimpleObject;
use chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub user_who_created: Option<String>
}
