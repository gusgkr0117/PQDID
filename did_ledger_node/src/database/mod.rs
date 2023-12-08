pub mod models;
mod schema;
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, QueryResult};
use diesel_async::{RunQueryDsl, *};
use dotenvy::dotenv;
use hex::FromHex;
use std::{env, time::SystemTime};

use crate::pqc_sign::types::{PublicKey, Signature};

use self::{models::DidDocuments, schema::diddocuments};

pub struct DidWallet {
    pub did: i64,
    pub public_key: Vec<u8>,
}

impl Into<DidDocuments> for DidWallet {
    fn into(self) -> DidDocuments {
        DidDocuments {
            did: self.did,
            user_did: None,
            doc_data: hex::encode(self.public_key),
            timestamp: SystemTime::now(),
            sig: None,
        }
    }
}

pub struct DidIssuedCert {
    pub did: i64,
    pub issuer_did: i64,
    pub json_type: serde_json::Value,
    pub signature: Signature,
}

impl Into<DidDocuments> for DidIssuedCert {
    fn into(self) -> DidDocuments {
        DidDocuments {
            did: self.did,
            user_did: Some(self.issuer_did),
            doc_data: self.json_type.to_string(),
            timestamp: SystemTime::now(),
            sig: Some(self.signature.value.to_vec()),
        }
    }
}

pub struct DidRegisteredCert {
    pub did: i64,
    pub user_did: i64,
    pub cert_did: i64,
    pub issuer_signature: Signature,
    pub signature: Signature,
}

impl Into<DidDocuments> for DidRegisteredCert {
    fn into(self) -> DidDocuments {
        DidDocuments {
            did: self.did,
            user_did: Some(self.user_did),
            doc_data: format!("{:#08X}", self.cert_did as u64)
                + "&"
                + hex::encode_upper(self.issuer_signature.value).as_str(),
            timestamp: SystemTime::now(),
            sig: Some(self.signature.value.to_vec()),
        }
    }
}

pub async fn establish_connection() -> AsyncPgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn create_wallet(
    conn: &mut AsyncPgConnection,
    wallet_info: DidWallet,
) -> QueryResult<usize> {
    let new_doc: DidDocuments = wallet_info.into();
    diesel::insert_into(diddocuments::table)
        .values(&new_doc)
        .execute(conn)
        .await
}

pub async fn issue_cert(
    conn: &mut AsyncPgConnection,
    did_issued_cert: DidIssuedCert,
) -> QueryResult<usize> {
    let new_doc: DidDocuments = did_issued_cert.into();
    diesel::insert_into(diddocuments::table)
        .values(&new_doc)
        .execute(conn)
        .await
}

pub async fn register_cert(
    conn: &mut AsyncPgConnection,
    did_registered_cert: DidRegisteredCert,
) -> QueryResult<usize> {
    let new_doc: DidDocuments = did_registered_cert.into();
    diesel::insert_into(diddocuments::table)
        .values(&new_doc)
        .execute(conn)
        .await
}

pub async fn read_pubkey(conn: &mut AsyncPgConnection, did: i64) -> QueryResult<PublicKey> {
    let did_doc = read_doc(conn, did).await?;
    // TODO: make a custom error type
    Ok(PublicKey::from_hex(did_doc.doc_data).unwrap())
}

pub async fn read_doc(conn: &mut AsyncPgConnection, did_input: i64) -> QueryResult<DidDocuments> {
    use self::schema::diddocuments::dsl::*;
    diddocuments
        .filter(did.eq(did_input))
        .first::<DidDocuments>(conn)
        .await
}
