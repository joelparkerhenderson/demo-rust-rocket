CREATE TABLE items (
  id uuid not null primary key,
  demo_boolean boolean, -- example: true
  demo_integer integer, -- example: 1
  demo_json json, -- example: '{"charlie": "delta"}'
  demo_numeric numeric(8,8), -- example: 1234.5678
  demo_text text, -- example: 'Alpha Bravo'
  demo_timestamp timestamp -- example: '2020-01-01T00:00:00'
);
