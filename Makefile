up:
	docker compose up -d --wait

schema:
	cargo run --example crab

reset:
	docker compose down -v