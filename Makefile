PORT=8000

db-up:
	make db-down
	docker-compose up -d --remove-orphans
db-down:
	docker-compose down

migration-install:
	cargo install sqlx-cli
migration-up:
	sqlx migrate run

docs-open-swagger:
	open http://localhost:$(PORT)/swagger-ui
docs-open-rapidoc:
	open http://localhost:$(PORT)/rapidoc
