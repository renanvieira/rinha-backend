CREATE OR REPLACE FUNCTION get_client_balance_and_transactions(client_id_param SMALLINT)
RETURNS JSONB
LANGUAGE plpgsql
AS $$
DECLARE
    v_client_exists BOOLEAN;
    v_latest_balance_record JSONB;
    v_transactions_record JSONB;
    v_limit INT;
    v_latest_balance INT;
BEGIN
    -- Check if the client exists
    SELECT EXISTS(SELECT 1 FROM dbapi.clients WHERE id = client_id_param) INTO v_client_exists;
    
    IF NOT v_client_exists THEN
        -- Return a JSON object indicating the client does not exist
        RETURN JSONB_BUILD_OBJECT('error', 'client_not_found');
    END IF;

    -- Get the latest balance for the client from the last transaction
    SELECT balance INTO v_latest_balance
    FROM dbapi.ledger
    WHERE client_id = client_id_param
    ORDER BY created_at DESC
    LIMIT 1;

    IF v_latest_balance IS NULL THEN
        v_latest_balance := 0;
    END IF;

    -- Get the client's limit from the clients table
    SELECT balance_limit INTO v_limit FROM dbapi.clients WHERE id = client_id_param;

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

