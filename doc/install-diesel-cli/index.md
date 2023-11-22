# Install diesel_cli (Diesel Command Line Interface)

## What to install

By default Diesel CLI depends on the following client libraries:

* libpq for the PostgreSQL backend

* libmysqlclient for the Mysql backend

* libsqlite3 for the SQlite backend

When we install Diesel CLI on a typical system that we'll be using for development or testing, then we prefer to install it for all the typical database features: Postgres, MySQL, SQLite.

When we install Diesel CLI for a production system that needs to be slim, we prefer to install it for Postgres and skip MySQL and SQLite.


## Install databases

To install the databases, you can use your system package manager.

### For macOS with brew

We prefer to install the full databases:

```sh
brew install postgresql libpq
brew install mysql mysql-client
brew install sqlite3
```

If you have an older version of a database, then you might see an alert, such as …

```sh
postgresql is already installed, it's just not migrated.
To migrate this formula, run:  
brew migrate postgresql@14
```

… Run the migrate command:

```sh
brew migrate postgresql@14
```

### For Debian-based systems with apt

Install dependencies on Debian:

```sh
sudo apt-get install -y libpq-dev
sudo apt-get install -y libmysqlclient-dev
sudo apt-get install -y libsqlite3-dev
```

Install databases:

```sh
sudo apt-get install -y postgresql-client-12
sudo apt-get install -y postgresql-12
sudo apt-get install -y mysql-client
sudo apt-get install -y mysql-server
sudo apt-get install -y sqlite
```

## Install Diesel CLI

Install:

```sh
cargo install diesel_cli
```
