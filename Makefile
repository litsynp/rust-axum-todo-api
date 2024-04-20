PORT=8000

watch\:install:
	cargo install cargo-watch
watch:
	cargo watch -x run
watch\:uninstall:
	cargo uninstall cargo-watch

db\:up:
	make db\:down
	docker-compose up -d --remove-orphans
db\:down:
	docker-compose down
db\:destroy:
	docker-compose down -v

migration\:install:
	cargo install sqlx-cli
migration\:up:
	sqlx migrate run

docs\:open\:swagger:
	open http://localhost:$(PORT)/swagger-ui
docs\:open\:rapidoc:
	open http://localhost:$(PORT)/rapidoc
