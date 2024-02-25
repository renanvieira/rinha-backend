CREATE OR REPLACE FUNCTION get_client_balance_and_transactions(client_id_param SMALLINT)
RETURNS JSONB
LANGUAGE plpgsql
AS $$
DECLARE
    v_latest_balance_record JSONB;
    v_transactions_record JSONB;
    v_limit INT;
    v_latest_balance INT;
    v_partition_name VARCHAR;
BEGIN
    -- Get the client's limit from the clients table
    v_limit = (SELECT balance_limit FROM dbapi.clients WHERE id = client_id_param);
    
    IF v_limit IS NULL THEN
        -- Return a JSON object indicating the client does not exist
        RETURN JSONB_BUILD_OBJECT('error', 'client_not_found');
    END IF;

    v_partition_name := 'dbapi.ledger_client_id_' || client_id_param;
    EXECUTE 'LOCK TABLE ' || v_partition_name || ' IN EXCLUSIVE MODE';

    -- Get the latest balance for the client from the last transaction
    SELECT balance INTO v_latest_balance
    FROM dbapi.ledger
    WHERE client_id = client_id_param
    ORDER BY created_at DESC
    LIMIT 1;

    IF v_latest_balance IS NULL THEN
        v_latest_balance := 0;
    END IF;

    -- Get the last few transactions for the client
    SELECT JSONB_AGG(sub.transaction ORDER BY sub.created_at DESC) INTO v_transactions_record
    FROM (
        SELECT JSONB_BUILD_OBJECT(
            'valor', amount,
            'tipo', operation,
            'descricao', descricao,
            'realizada_em', created_at
        ) AS transaction, created_at
        FROM dbapi.ledger
        WHERE client_id = client_id_param
        ORDER BY created_at DESC
        LIMIT 10
    ) sub;

    IF v_transactions_record IS NULL THEN
        v_transactions_record := JSONB_BUILD_ARRAY();
    END IF;

    -- Build the JSON structure with the balance information and the transactions
    v_latest_balance_record := JSONB_BUILD_OBJECT(
        'saldo', JSONB_BUILD_OBJECT(
            'total', v_latest_balance,
            'data_extrato', NOW(),
            'limite', v_limit
        ),
        'ultimas_transacoes', v_transactions_record
    );

    RETURN v_latest_balance_record;
END;
$$;

