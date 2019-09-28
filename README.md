# forrs-data
Contains the data model and corresponding SQL scripts for forrs.

## Setup
We're currently using migrant cli for our database migrations; to install:

    cargo install migrant --features mysql

Then, copy Migrant.example.toml to Migrant.toml and add your local database info.

Starting with a fresh database, run:

    migrant setup

To add the `__migrant_migrations` table to your database, followed by:

    migrant apply --all

To apply all available migrations.
