CREATE TABLE IF NOT EXISTS DidDocuments (
    did bigint PRIMARY KEY,
    user_did bigint not null,
    doc_data text not null,
    timestamp timestamp not null,
    sig bytea
);