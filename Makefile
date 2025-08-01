include .env
export

test:
	sqlx database drop -y
	sqlx database create
	sqlx migrate run
	for file in tests/seedings/*.sql; do \
		psql "$$DATABASE_URL" -f "$$file"; \
	done
	cargo test
