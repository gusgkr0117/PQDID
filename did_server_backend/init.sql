CREATE TABLE IF NOT EXISTS Users (
    user_id text PRIMARY KEY,
    passwd text not null,
    username text not null,
    wallet_did bigint not null
);

CREATE TABLE IF NOT EXISTS Certificates (
    id SERIAL PRIMARY KEY,
    did bigint,
    user_id text not null,
    issuer_id text not null,
    cert_did bigint not null,
    cert_info text,
    issuer_sig bytea,
    stat int not null
);

CREATE TABLE IF NOT EXISTS PreCertificates (
    did bigint PRIMARY KEY,
    issuer_id text not null,
    template text not null,
    cert_name text not null
);