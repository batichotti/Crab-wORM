up:
	docker compose up -d --wait

schema:
	cargo run --example crab && docker compose exec -T db sh -c 'psql -U "$$POSTGRES_USER" -d "$$POSTGRES_DB" -v ON_ERROR_STOP=1' < schema.sql

reset:
	docker compose down -v