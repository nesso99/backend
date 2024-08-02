run-user:
  cargo run -p user

migrate-user:
  sqlx migrate run --source ./crates/user/migrations

install-sqlx-cli:
  cargo install sqlx-cli

database-create:
  sqlx database create

database-drop:
  sqlx database drop

database-reset:
  just database-drop
  just database-create
