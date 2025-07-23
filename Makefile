test:
	sqlx database drop -y
	sqlx database create
	sqlx migrate run
	cargo test
