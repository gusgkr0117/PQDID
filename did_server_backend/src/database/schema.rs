// @generated automatically by Diesel CLI.

diesel::table! {
    certificates (id) {
        id -> Int4,
        did -> Nullable<Int8>,
        user_id -> Text,
        issuer_id -> Text,
        cert_did -> Int8,
        cert_info -> Nullable<Text>,
        issuer_sig -> Nullable<Bytea>,
        stat -> Int4,
    }
}

diesel::table! {
    precertificates (did) {
        did -> Int8,
        issuer_id -> Text,
        template -> Text,
        cert_name -> Text,
    }
}

diesel::table! {
    users (username) {
        user_id -> Text,
        passwd -> Text,
        username -> Text,
        wallet_did -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(certificates, precertificates, users,);
