.PHONY: dup

	docker-compose up -d backend-development
up-dev:
down:
	docker compose down
exec:
	docker exec -it $(CONTAINER_ID) /bin/bash
