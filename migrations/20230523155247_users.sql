CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    email      VARCHAR(255) NOT NULL,
    password   VARCHAR(255) NOT NULL,
    nickname   VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

ALTER TABLE users
    ADD CONSTRAINT users_email_uq
        UNIQUE (email);

CREATE INDEX users_email_ix
    ON users (email)
    WHERE
        deleted_at IS NULL;
