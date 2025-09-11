-- Create database if it doesn't exist
DO
$$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_database WHERE datname = 'nr11') THEN
      PERFORM dblink_exec('dbname=postgres', 'CREATE DATABASE nr11');
   END IF;
END
$$;

-- Connect to the database
\c nr11;

-- Create tables if they don't exist
CREATE TABLE IF NOT EXISTS "user" (
    id SERIAL PRIMARY KEY,
    user_name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    notification_id VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS product (
    id SERIAL PRIMARY KEY,
    image TEXT NOT NULL,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    price REAL NOT NULL,
    currency VARCHAR NOT NULL
);
