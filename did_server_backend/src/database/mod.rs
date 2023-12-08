pub mod models;
pub mod schema;

use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, QueryResult};
use diesel_async::*;
use dotenvy::dotenv;
use std::env;

use crate::error::Error;
use crate::pqc_sign::types::Signature;

use self::models::{Certificates, PreCertificates, Users};
use self::schema::{certificates, precertificates, users};

pub async fn establish_connection() -> AsyncPgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn create_user(conn: &mut AsyncPgConnection, user_info: Users) -> QueryResult<usize> {
    diesel::insert_into(users::table)
        .values(&user_info)
        .execute(conn)
        .await
}

pub async fn verify_user(
    conn: &mut AsyncPgConnection,
    user_id_: String,
    passwd_: String,
) -> QueryResult<bool> {
    use self::schema::users::dsl::*;
    let existence = diesel::dsl::select(diesel::dsl::exists(
        users
            .filter(user_id.eq(user_id_))
            .filter(passwd.eq(passwd_)),
    ))
    .get_result(conn)
    .await?;

    Ok(existence)
}

pub async fn get_user_did(conn: &mut AsyncPgConnection, user_id_: String) -> QueryResult<i64> {
    use self::schema::users::dsl::*;
    let the_user = users
        .filter(user_id.eq(user_id_))
        .first::<Users>(conn)
        .await?;
    Ok(the_user.wallet_did)
}

pub async fn create_precert(
    conn: &mut AsyncPgConnection,
    precert_info: PreCertificates,
) -> QueryResult<usize> {
    diesel::insert_into(precertificates::table)
        .values(&precert_info)
        .execute(conn)
        .await
}

pub async fn get_precert_from_did(
    conn: &mut AsyncPgConnection,
    precert_did: i64,
) -> Result<PreCertificates, Error> {
    use self::schema::precertificates::dsl::*;
    let precert = precertificates
        .filter(did.eq(precert_did))
        .first::<PreCertificates>(conn)
        .await?;
    Ok(precert)
}

pub async fn create_cert(
    conn: &mut AsyncPgConnection,
    cert_info: Certificates,
) -> QueryResult<usize> {
    diesel::insert_into(certificates::table)
        .values(&cert_info)
        .execute(conn)
        .await
}

pub async fn approve_cert(
    conn: &mut AsyncPgConnection,
    cert_id: i32,
    sig: Signature,
) -> QueryResult<usize> {
    use self::schema::certificates::dsl::{id, issuer_sig, stat};
    diesel::update(certificates::table)
        .filter(id.eq(cert_id))
        .set((issuer_sig.eq(sig.value.to_vec()), stat.eq(1)))
        .execute(conn)
        .await
}

/// Update following column
/// + did (did of the certificate)
/// + stat (status of the certificate) -> 2
pub async fn register_cert(
    conn: &mut AsyncPgConnection,
    cert_id: i32,
    cert_did: i64,
) -> QueryResult<usize> {
    use self::schema::certificates::dsl::{did, id, stat};
    diesel::update(certificates::table)
        .filter(id.eq(cert_id))
        .set((stat.eq(2), did.eq(Some(cert_did))))
        .execute(conn)
        .await
}

pub async fn get_cert_info(
    conn: &mut AsyncPgConnection,
    cert_id: i32,
) -> QueryResult<Option<String>> {
    use self::schema::certificates::dsl::*;
    let cert = certificates
        .filter(id.eq(cert_id))
        .first::<Certificates>(conn)
        .await?;
    Ok(cert.cert_info)
}

pub async fn get_cert_from_id(
    conn: &mut AsyncPgConnection,
    cert_id: i32,
) -> Result<Certificates, Error> {
    use self::schema::certificates::dsl::*;
    let cert = certificates
        .filter(id.eq(cert_id))
        .first::<Certificates>(conn)
        .await?;
    Ok(cert)
}
