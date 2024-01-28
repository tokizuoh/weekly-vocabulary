.PHONY: dup

up-dev:
	docker-compose up -d backend-development
down:
	docker compose down
exec:
	docker exec -it $(CONTAINER_ID) /bin/bash
