.PHONY: run
start:
	docker-compose exec todo-app-backend /bin/bash -c "cd /app/server && cargo run"

.PHONY: migrate
migrate:
	docker-compose exec todo-app-backend /bin/bash -c "cd /app/server/schema && diesel migration run"

# .PHONY: generate
# generate:
# 	docker-compose exec app go generate ./...

# .PHONY: migrate-create
# migrate-create:
# 	docker-compose exec app migrate -ext sql -dir migrations ${FILENAME}

# .PHONY: migrate-up
# migrate-up:
# 	docker-compose exec app migrate --source file://migrations --database ${DB_CONN} up

# .PHONY: migrate-down
# migrate-down:
# 	docker-compose exec app migrate --source file://migrations --database ${DB_CONN} down 1