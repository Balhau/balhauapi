-- Your SQL goes here
CREATE TABLE T_AUTHOR (
  id_author INTEGER PRIMARY KEY,
  f_id_author INTEGER,
  f_name TEXT,
  f_country TEXT
);

CREATE TABLE T_DIRECTORY (
 id_dir INTEGER PRIMARY KEY,
 f_id_dir INTEGER,
 f_path TEXT NOT NULL,
 f_isclouddir INTEGER DEFAULT 0
);


CREATE TABLE T_ITEM (
  id_item INTEGER PRIMARY KEY,
  f_id_item INTEGER,
  f_etag TEXT,
  f_item_fileformat INTEGER,
  f_item_filetype INTEGER,
  f_item_category_flag INTEGER,
  f_service_id INTEGER,
  f_lang INTEGER DEFAULT -1,
  f_pages_number INTEGER,
  f_current_page INTEGER,--page courante
  f_last_read INTEGER,--derniere lecture
  f_publication_date TEXT,
  f_publisher TEXT,
  f_title TEXT,
  f_cover_uri TEXT,
  f_description TEXT,
  f_id_dir INTEGER,
  f_internal_uri TEXT,
  f_external_uri TEXT,
  f_documenttime INTEGER,--19 janvier 2038,
  f_filesize INTEGER,
  f_drminfo TEXT,
  f_expiration_date INTEGER,
  f_ade_page_index TEXT,
  f_author_id INTEGER,
  f_toupdate INTEGER, -- 1 le fichier est OK 0 to update
  f_isvalid INTEGER default 1, f_cf1 TEXT,
  f_cf2 TEXT, f_coverisvalid INTEGER default 1,
  f_islastpage INTEGER default 0,
  f_preorder_date INTEGER -- le fichier est corrompu mettre � z�ro.
);
