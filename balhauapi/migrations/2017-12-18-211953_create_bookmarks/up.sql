-- Your SQL goes here
CREATE TABLE bookmarks (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  b64icon TEXT,
  created TIMESTAMP NOT NULL,
  url VARCHAR NOT NULL
)
