Databases

Rocket includes built-in, ORM-agnostic support for databases.

See source:
<src/demos/database.rs>

See Rocket API:
<https://rocket.rs/v0.5/guide/state/#databases>


## Create a database

Choose a name for your database, such as `demo_rust_rocket_sqlite`. 

Create a directory, a database, a table, and a row of data, such as:

```sh
mkdir databases
cd databases
sqlite3 demo_rust_rocket.sqlite
sqlite> create table items(id int, name text, done boolean);
sqlite> insert into items values (1, "Call Alice", false);
```


## Add sqlx SQLite dependency

Choose your database(s) from the supported database driver list. For example,
this demo uses the database SQLite and the driver `sqlx_sqlite``.

Add rocket_db_pools as a dependency in `Cargo.toml` with respective database
driver feature(s) enabled:

```toml
rocket_db_pools = { version = "0.1.0", features = ["sqlx_sqlite"] }
```


## Configure URL

Configure at least a URL for the database under databases.$name,
such as in the file `Rocket.toml`:

```toml
[default.databases.demo_rust_rocket_sqlite]
url = "/path/to/demo_rust_rocket.sqlite"
```


## Add code

Add code to file `main.rs`:

```rust
use rocket_db_pools::Database;
use rocket_db_pools::sqlx;

#[derive(Database)]
#[database("demo_rust_rocket_sqlite")]
pub struct DemoRustRocketSqliteConnection(sqlx::SqlitePool);
```
