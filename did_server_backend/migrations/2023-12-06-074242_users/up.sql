-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Users (
    user_id text PRIMARY KEY,
    passwd text not null,
    username text not null,
    wallet_did bigint not null
)