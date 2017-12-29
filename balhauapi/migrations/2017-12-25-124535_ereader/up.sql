-- Your SQL goes here
CREATE TABLE ereaderauthor (
  id_author SERIAL PRIMARY KEY,
  f_id_author INTEGER,
  f_name TEXT,
  f_country TEXT
);

CREATE TABLE ereaderdirectory (
 id_dir SERIAL PRIMARY KEY,
 f_id_dir INTEGER,
 f_path TEXT NOT NULL,
 f_isclouddir INTEGER DEFAULT 0
);

CREATE TABLE ereaderitem (
  id_item SERIAL PRIMARY KEY,
  f_id_item INTEGER,
  f_pages_number INTEGER,
  f_current_page INTEGER,
  f_last_read INTEGER,
  f_publication_date TEXT,
  f_publisher TEXT,
  f_title TEXT,
  f_description TEXT,
  f_author_id INTEGER
);

