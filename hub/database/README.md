# Hub Database

The Database is based on [PostgreSQL](https://www.postgresql.org/) +
[Timescale](https://www.timescale.com/). It contains time-series of
the data fetched by the WebThings. It provides metrics, and can be
used to automate some parts of the house.

This directory contains documentation and migration scripts to
maintain the database.

The database migration scripts are run by
[Diesel](https://diesel.rs/).
