use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::diddocuments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DidDocuments {
    pub did: i64,
    pub user_did: Option<i64>,
    pub doc_data: String,
    pub sig: Option<Vec<u8>>,
}
