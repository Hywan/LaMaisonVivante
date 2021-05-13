# [TimescaleDB](https://github.com/timescale/timescaledb)

## Installation

Install Timescale:

* [on macOS](https://docs.timescale.com/timescaledb/latest/how-to-guides/install-timescaledb/self-hosted/macos/installation-homebrew/#homebrew)

  ```sh
  $ brew tap timescale/tap
  $ brew install timescaledb
  $
  $ timescaledb_move.sh
  $
  $ timescaledb-tune
  $ # or 
  $ echo "shared_preload_libraries='timescaledb'" >> /opt/homebrew/var/postgres/postgresql.conf
  
  $ # Disable telemetry
  $ echo "timescaledb.telemetry_level=off" >> /opt/homebrew/var/postgres/postgresql.conf
  $
  $ brew services restart postgresql
  $
  $ createuser -s postgres
  ```
  
* [on Debian](https://docs.timescale.com/timescaledb/latest/how-to-guides/install-timescaledb/self-hosted/debian/installation-apt-debian/#apt-installation-debian)

  ```sh
  $ echo "deb http://apt.postgresql.org/pub/repos/apt/ $(lsb_release -c -s)-pgdg main" | sudo tee /etc/apt/sources.list.d/pgdg.list
  $ wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
  $ sudo apt-get update
  $
  $ sudo sh -c "echo 'deb https://packagecloud.io/timescale/timescaledb/debian/ `lsb_release -c -s` main' > /etc/apt/sources.list.d/timescaledb.list"
  $ wget --quiet -O - https://packagecloud.io/timescale/timescaledb/gpgkey | sudo apt-key add -
  $ sudo apt-get update
  $
  $ sudo apt-get install timescaledb-2-postgresql-:pg_version:
  ```

## Set up

To set up Timescale:

```sh
$ psql -U postgres -h localhost
#
# ; Disable the telemetry.
# ALTER [SYSTEM | DATABASE | USER] { *db_name* | *role_specification* } SET timescaledb.telemetry_level=off
#
# ; Create the database.
# CREATE database la_maison_vivante;
#
# ; Connect to the database.
# \c la_maison_vivante
#
# ; Load/create the extension.
# CREATE EXTENSION IF NOT EXISTS timescaledb;
```

## Tables

```sql
CREATE TABLE IF NOT EXISTS my_table ( … time TIMESTAMP WITHOUT TIME ZONE NOT NULL … );

SELECT create_hypertable('my_table', 'time');
```

# [Grafana](https://grafana.com/grafana/)

## Installation

Install Timescale:

* [on macOS](https://grafana.com/grafana/download?pg=get&plcmt=selfmanaged-box1-cta1&platform=mac)

  ```sh
  $ brew install grafana
  ```


# Misc

https://grafana.com/docs/grafana/latest/getting-started/getting-started/
https://docs.timescale.com/timescaledb/latest/getting-started/query-data/##time-bucket-gapfill
https://docs.timescale.com/timescaledb/latest/tutorials/grafana/create-dashboard-and-panel/#build-a-new-dashboard

# [Diesel](https://diesel.rs/)

## Set up

```shell
$ # Install Diesel.
$ cargo install diesel_cli --no-default-features --features postgres
$
$ # Define where's the database.
$ export DATABASE_URL="postgres://username:password@localhost/la_maison_vivante"
$
$ # Set up, do it only once if the `migrations` directory is absent.
$ diesel setup \
      --migration-dir path/to/hub/database/migrations \
      --database-url $DATABASE_URL
$ 
$ # Create a migration script, if needed.
$ diesel migration \
      --migration-dir path/to/hub/database/migrations \
      generate create_electricity
$
$ # Run the migrations.
$ diesel migration \
      --migration-dir path/to/hub/database/migrations \
      run
```

## 
