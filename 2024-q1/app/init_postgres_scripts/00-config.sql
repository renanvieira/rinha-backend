ALTER SYSTEM SET
 max_connections = '350';
ALTER SYSTEM SET
 shared_buffers = '75MB';
ALTER SYSTEM SET
 effective_cache_size = '225MB';
ALTER SYSTEM SET
 maintenance_work_mem = '19200kB';
ALTER SYSTEM SET
 checkpoint_completion_target = '0.9';
ALTER SYSTEM SET
 wal_buffers = '2304kB';
ALTER SYSTEM SET
 default_statistics_target = '100';
ALTER SYSTEM SET
 random_page_cost = '1.1';
ALTER SYSTEM SET
 effective_io_concurrency = '200';
ALTER SYSTEM SET
 work_mem = '109kB';
ALTER SYSTEM SET
 huge_pages = 'off';
ALTER SYSTEM SET
 min_wal_size = '1GB';
ALTER SYSTEM SET
 max_wal_size = '4GB';
ALTER SYSTEM SET 
 fsync = 'off'
ALTER SYSTEM SET
 full_page_writes = 'off';
