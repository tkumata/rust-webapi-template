-- Your SQL goes here
CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT users_email_key UNIQUE (email),
	CONSTRAINT users_pkey PRIMARY KEY (id)
);

CREATE TABLE user_tokens (
    id UUID NOT NULL,
    user_id UUID NOT NULL,
    "token" TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    CONSTRAINT user_tokens_pkey PRIMARY KEY (id),
    CONSTRAINT user_tokens_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id)
);
