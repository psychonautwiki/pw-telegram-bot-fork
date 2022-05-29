use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQuery {
    pub id: InlineQueryId,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

impl Into<InlineQueryId> for InlineQuery {
    fn into(self) -> InlineQueryId {
        self.id
    }
}
