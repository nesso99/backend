run-user:
  cargo watch -x 'run -p user'

install-tools:
  cargo install sqlx-cli
  cargo install cargo-watch

db-migrate:
  sqlx migrate run

db-create:
  sqlx database create

db-drop:
  sqlx database drop

db-reset:
  sqlx database reset
