CREATE TABLE captures (
    id SERIAL PRIMARY KEY,
    uuid uuid UNIQUE NOT NULL,
    url text NOT NULL,
    time timestamp NOT NULL,
    owner integer NOT NULL,
    public boolean NOT NULL
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name text UNIQUE NOT NULL,
    passhash text NOT NULL
);
