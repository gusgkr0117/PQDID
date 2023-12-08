CREATE TABLE IF NOT EXISTS Users (
    user text PRIMARY KEY,
    password text not null,
    wallet_did bigint not null
);

CREATE TABLE IF NOT EXISTS Certificates (
    id SERIAL PRIMARY KEY,
    cert_did bigint,
    owner text not null,
    issuer text not null,
    json_did bigint not null,
    cert_info text,
    status int not null
);

CREATE TABLE IF NOT EXISTS PreCertificates (
    did: bigint PRIMARY KEY,
    issuer_id: text not null,
    template: text not null,
    cert_name: text not null
);