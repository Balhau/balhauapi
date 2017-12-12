-- Your SQL goes here
CREATE TABLE blog_posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  post_created TIMESTAMP NOT NULL,
  post_updated TIMESTAMP NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)

