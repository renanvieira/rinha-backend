CREATE TABLE IF NOT EXISTS "dbapi"."clients"(
    id smallint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    balance_limit integer NOT NULL,
    balance integer NOT NULL DEFAULT 0
);

INSERT INTO dbapi.clients(balance_limit) VALUES(100000);
INSERT INTO dbapi.clients(balance_limit) VALUES(80000);
INSERT INTO dbapi.clients(balance_limit) VALUES(1000000);
INSERT INTO dbapi.clients(balance_limit) VALUES(10000000);
INSERT INTO dbapi.clients(balance_limit) VALUES(500000);

