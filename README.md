# eve

![Eve, by Stable Diffusion](eve.png)

## dotenv

Some environment variables are needed for the development of the application.
To set up, copy `.envrc.sample` to `.envrc` and edit the variables.

## Database schema

Run:

```sh
docker run -v "$PWD/schema:/output" --net="host" schemaspy/schemaspy:snapshot -t pgsql -host localhost:5432 -db eve -u postgres -p ${POSTGRESQL_ROOT_PASSWORD}
```

to generate schema and ER digram in `./schema`.

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
