.PHONY: up-dev, down, exec

up-dev:
	docker-compose up -d application-development mysql-development
down:
	docker compose down
exec:
	docker exec -it $(CONTAINER_ID) /bin/bash
