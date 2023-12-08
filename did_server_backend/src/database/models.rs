use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub user_id: String,
    pub passwd: String,
    pub username: String,
    pub wallet_did: i64,
}

impl Users {
    pub fn new(user_id: String, passwd: String, username: String, wallet_did: i64) -> Self {
        Users {
            user_id,
            passwd,
            username,
            wallet_did,
        }
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::certificates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Certificates {
    pub id: i32,
    pub did: Option<i64>,
    pub user_id: String,
    pub issuer_id: String,
    pub cert_did: i64,
    pub cert_info: Option<String>,
    pub issuer_sig: Option<Vec<u8>>,
    pub stat: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::precertificates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PreCertificates {
    pub did: i64,
    pub issuer_id: String,
    pub template: String,
    pub cert_name: String,
}
