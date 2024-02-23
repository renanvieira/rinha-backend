CREATE UNLOGGED TABLE "dbapi"."ledger" (
    id integer GENERATED ALWAYS AS IDENTITY,
    client_id SMALLINT NOT NULL,
    amount INTEGER NOT NULL,
    operation CHAR(1) NOT NULL,
    descricao VARCHAR(10) NOT NULL,
    balance INTEGER NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT (now() at time zone 'utc'),
    PRIMARY KEY (client_id, id)
) PARTITION BY LIST (client_id);

CREATE INDEX ON "dbapi"."ledger" (client_id, created_at DESC) INCLUDE(balance);

CREATE TABLE dbapi.ledger_client_id_1 PARTITION OF dbapi.ledger
    FOR VALUES IN (1);

CREATE INDEX ON "dbapi"."ledger_client_id_1" (client_id, created_at DESC) INCLUDE(balance);

CREATE TABLE dbapi.ledger_client_id_2 PARTITION OF dbapi.ledger
    FOR VALUES IN (2);

CREATE INDEX ON "dbapi"."ledger_client_id_2" (client_id, created_at DESC) INCLUDE(balance);

CREATE TABLE dbapi.ledger_client_id_3 PARTITION OF dbapi.ledger
    FOR VALUES IN (3);

CREATE INDEX ON "dbapi"."ledger_client_id_3" (client_id, created_at DESC) INCLUDE(balance);

CREATE TABLE dbapi.ledger_client_id_4 PARTITION OF dbapi.ledger
    FOR VALUES IN (4);

CREATE INDEX ON "dbapi"."ledger_client_id_4" (client_id, created_at DESC) INCLUDE(balance);

CREATE TABLE dbapi.ledger_client_id_5 PARTITION OF dbapi.ledger
    FOR VALUES IN (5);

CREATE INDEX ON "dbapi"."ledger_client_id_5" (client_id, created_at DESC) INCLUDE(balance);
