run:
	cargo run main

run-mysql:
	docker-compose up -d

down-mysql:
	docker-compose down -d