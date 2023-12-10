// @generated automatically by Diesel CLI.

diesel::table! {
    diddocuments (did) {
        did -> Int8,
        user_did -> Nullable<Int8>,
        doc_data -> Text,
        sig -> Nullable<Bytea>,
    }
}
