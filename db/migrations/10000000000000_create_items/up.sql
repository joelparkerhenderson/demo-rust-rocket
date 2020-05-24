CREATE TABLE items (
  id int not null primary key,
  demo_boolean boolean, -- example: true
  demo_int int, -- example: 1
  demo_json json, -- example: '{"charlie": "delta"}'
  demo_text text, -- example: 'Alpha Bravo'
  demo_timestamp timestamp -- example: '2020-01-01T00:00:00'
);
