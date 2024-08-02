run-user:
  cargo run -p user

install-sqlx-cli:
  cargo install sqlx-cli

database-create:
  sqlx database create

database-drop:
  sqlx database drop
