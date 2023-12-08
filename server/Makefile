SHELL := /bin/bash

# db_url := postgres://postgres:postgres@192.168.68.102:5432/sadhana_pro

# run_server:
# 	\
# 		RUST_BACKTRACE=full \
# 		cargo run --bin server

run: 
	RUST_BACKTRACE=full \
		cargo run --bin server

create_migration:
	diesel migration generate $(name) --migration-dir=migrations

migrate:
	diesel migration run --migration-dir=migrations

redo_migrate:
	diesel migration redo --migration-dir=migrations

undo_migrate:
	diesel migration revert --migration-dir=migrations

reset_db:
	diesel database reset --migration-dir=migrations

gen_schema:
	diesel print-schema > src/schema.rs

# test:
# 	\
# 		JWT_KEY= \
# 		cargo test $(T) -- --nocapture --test-threads=1

# lint:
# 	@rustup component add clippy 2> /dev/null
# 	cargo clippy --all-targets --all-features -- -D warnings

# non-file target for make
.PHONY: run create_migration migrate redo_migrate reset_db gen_schema
