--: Url(id, short_code, url, created_at, updated_at)
--: UrlWithStats(id, short_code, url, created_at, updated_at, access_count)

--! get_mapping (short_code) : Url
SELECT id, short_code, url, created_at, updated_at
FROM urls
WHERE urls.short_code = :short_code;

--! get_url_with_stats(short_code) : UrlWithStats
SELECT u.id as id, short_code, url, created_at, updated_at, amount as access_count
FROM urls u
LEFT JOIN visits v ON u.id = v.url_id
WHERE u.short_code = :short_code;

--! insert_url
INSERT INTO urls (id, short_code, url, created_at, updated_at)
VALUES (:id, :short_code, :url, :created_at, :updated_at);

--! update_url : Url
UPDATE urls
SET url = :url
WHERE short_code = :code
RETURNING *;

--! delete_url (short_code)
DELETE FROM urls
WHERE short_code = :short_code;
