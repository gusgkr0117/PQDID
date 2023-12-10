CREATE TABLE IF NOT EXISTS DidDocuments (
    did bigint PRIMARY KEY,
    user_did bigint,
    doc_data text not null,
    sig bytea
);