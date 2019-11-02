# forrs-data
Contains the data model and corresponding SQL scripts for forrs.

## Setup
We're currently using migrant cli for our database migrations; to install:

    cargo install migrant --features postgres

Then, copy Migrant.example.toml to Migrant.toml and add your local database info.

> Note: If you're on a 32-bit system and don't want to enable the big-numbers
> feature, change `migrations_location` to `migrations-32` as well.

Starting with a fresh database, run:

    migrant setup

To add the `__migrant_migrations` table to your database, followed by:

    migrant apply --all

To apply all available migrations.

## Current state
While I don't have the time to really document it right now, this is now a
workspace; the `forrs-data` project is the actual data model, while
`forrs-stm` is our "Struct Table Mapper", a strongly typed DB client, and
the crate also contains traits for working with data from the db.
