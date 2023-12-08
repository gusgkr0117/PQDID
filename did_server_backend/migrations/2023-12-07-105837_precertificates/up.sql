-- Your SQL goes here
CREATE TABLE IF NOT EXISTS PreCertificates (
    did bigint PRIMARY KEY,
    issuer_id text not null,
    template text not null,
    cert_name text not null
)