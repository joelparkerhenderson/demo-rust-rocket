GRANT ALL ON DATABASE demo_rust_rocket TO demo_rust_rocket;

GRANT CONNECT ON DATABASE demo_rust_rocket TO demo_rust_rocket;
GRANT USAGE ON SCHEMA public TO demo_rust_rocket;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO demo_rust_rocket;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO demo_rust_rocket;
