CREATE TABLE IF NOT EXISTS users (
    uuid TEXT PRIMARY KEY,
    cloak TEXT DEFAULT "",
    hat TEXT DEFAULT "",
    cloaks TEXT DEFAULT '[]',
    hats TEXT DEFAULT '[]'
);