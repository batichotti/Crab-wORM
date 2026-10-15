up:
	docker compose up -d --wait

schema:
	cargo run --example crab

schema-reset:
	CRAB_RESET=1 cargo run --example crab

reset:
	docker compose restart api

down:
	docker compose down -v