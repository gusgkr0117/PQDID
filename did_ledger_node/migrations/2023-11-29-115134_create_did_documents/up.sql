-- Your SQL goes here
CREATE TABLE IF NOT EXISTS DidDocuments (
    did bigint PRIMARY KEY,
    user_did bigint,
    doc_data text not null,
    timestamp timestamp not null,
    sig bytea
)