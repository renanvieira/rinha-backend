CREATE OR REPLACE FUNCTION insert_transaction(
    p_client_id SMALLINT,
    p_amount INTEGER,
    p_operation CHAR,
    p_descricao VARCHAR(10)
) RETURNS JSONB
 LANGUAGE plpgsql
AS $$
DECLARE
    v_balance INTEGER;
    v_latest_balance INTEGER;
    v_return_object JSONB;
    v_account_limit INTEGER;
    v_partition_name VARCHAR;
BEGIN

    v_partition_name := 'dbapi.ledger_client_id_' || p_client_id;

    EXECUTE 'LOCK TABLE ' || v_partition_name || ' IN ROW EXCLUSIVE MODE';
    
    v_account_limit := (SELECT balance_limit FROM dbapi.clients WHERE id = p_client_id);

    IF v_account_limit IS NULL THEN
        RETURN JSONB_BUILD_OBJECT('error','client_not_found');
    END IF;


    -- Get the latest balance for the client from the last transaction
    SELECT balance INTO v_latest_balance
    FROM dbapi.ledger
    WHERE client_id = p_client_id
    ORDER BY created_at DESC
    LIMIT 1;

    IF v_latest_balance IS NULL THEN
        v_latest_balance := 0;
    END IF;

    -- Adjust the balance based on the operation
    IF p_operation = 'c' THEN
        v_balance := v_latest_balance + p_amount;
    ELSIF p_operation = 'd' THEN
        v_balance := v_latest_balance - p_amount;
    ELSE
        RETURN JSONB_BUILD_OBJECT('error', 'invalid_operation');
    END IF;

    IF v_balance < (v_account_limit * -1) THEN
        RETURN JSONB_BUILD_OBJECT('error', 'not_enough_limit');
    END IF;

    -- Insert the transaction with the calculated balance into the ledger
    INSERT INTO dbapi.ledger (client_id, amount, operation, descricao, balance)
    VALUES (p_client_id, p_amount, p_operation, p_descricao, v_balance);
    
    v_return_object := JSONB_BUILD_OBJECT(
        'limite', v_account_limit,
        'saldo', v_balance
    );

    RETURN v_return_object;

EXCEPTION WHEN OTHERS THEN
    -- Rollback the transaction in case of any failure is handled automatically by PostgreSQL
    -- Optionally, re-raise the exception or handle it accordingly
    RAISE;
END;
$$;
