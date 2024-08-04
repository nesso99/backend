run-user:
  cargo run -p user


install-sqlx-cli:
  cargo install sqlx-cli

db-migrate:
  sqlx migrate run

db-create:
  sqlx database create

db-drop:
  sqlx database drop

db-reset:
  sqlx database reset
