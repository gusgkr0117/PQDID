-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Certificates (
    id SERIAL PRIMARY KEY,
    did bigint,
    user_id text not null,
    issuer_id text not null,
    cert_did bigint not null,
    cert_info text,
    issuer_sig bytea,
    stat int not null
)