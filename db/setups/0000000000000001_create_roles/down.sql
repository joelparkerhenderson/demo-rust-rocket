REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM demo_rust_rocket;
REVOKE ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public FROM demo_rust_rocket;
REVOKE ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public FROM demo_rust_rocket;
REVOKE USAGE ON SCHEMA public FROM demo_rust_rocket;
REVOKE ALL PRIVILEGES ON DATABASE demo_rust_rocket FROM demo_rust_rocket;
DROP OWNED BY demo_rust_rocket;
REASSIGN OWNED BY demo_rust_rocket TO postgres;
DROP ROLE demo_rust_rocket;
