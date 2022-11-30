-- name: create-table-accounts
CREATE TABLE accounts (
  id BIGSERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  password TEXT NOT NULL,
  token TEXT NOT NULL,
  email TEXT NOT NULL,
  friends TEXT[],
  snowflake TEXT NOT NULL,
  avatar TEXT,
);

-- name: create-table-messages
CREATE TABLE messages (
  id INTEGER BIGSERIAL PRIMARY KEY,
  snowflake TEXT NOT NULL,
  from TEXT NOT NULL,
  to TEXT NOT NULL,
  content TEXT NOT NULL,
);

-- name: create-account
INSERT INTO accounts (snowflake, name, password, token, email, friends) VALUES ($1, $2, $3, $4, $5, '{}');

-- name: read-account
SELECT snowflake, name, email, friends, avatar FROM accounts WHERE token=$1;

-- name: find-account
SELECT snowflake, name, friends, avatar FROM accounts WHERE snowflake=$1;

-- name: search-account
SELECT token FROM accounts WHERE email=$1 AND password=$2;

-- name: update-account-name
UPDATE accounts SET name=$1 WHERE token=$2;

-- name: update-account-password
UPDATE accounts SET password=$1 WHERE token=$2;

-- name: update-account-email
UPDATE accounts SET email=$1 WHERE token=$2;

-- name: update-account-token
UPDATE accounts SET token=$1 WHERE token=$2;

-- name: update-account-friend-add
UPDATE accounts SET friends=ARRAY_APPEND(friends,$1) WHERE token=$2;

-- name: update-account-friend-remove
UPDATE accounts SET friends=ARRAY_REMOVE(friends,$1) WHERE token=$2;

-- name: delete-account
DELETE FROM accounts WHERE token=$1;