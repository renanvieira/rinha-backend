CREATE UNLOGGED TABLE "dbapi"."ledger" (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    client_id SMALLINT NOT NULL,
    amount INTEGER NOT NULL,
    operation CHAR(1) NOT NULL CHECK (operation IN ('c', 'd')), -- assuming 'c' for credit, 'd' for debit
    descricao VARCHAR(10) NOT NULL,
    balance INTEGER NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT (now() at time zone 'utc'),
    FOREIGN KEY (client_id) REFERENCES dbapi.clients(id)
);

CREATE INDEX ON "dbapi"."ledger" (client_id, created_at DESC) INCLUDE(balance);
