# eve

## dotenv

Some environment variables are needed for the development of the application.
To set up, copy `.envrc.sample` to `.envrc` and edit the variables.

## Server

Make sure `sqlx-cli` is available by `cargo install sqlx-cli`.

```sh
# Launch required dependencies (MinIO and PostgreSQL)
docker compose up -d # or docker-compose up -d

cd server

# Run migration
sqlx database create
sqlx migrate run

# Start server
cargo run
```

## App

```sh
cd app
yarn
yarn dev
```
