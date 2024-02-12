.PHONY: generate

generate:
	./node_modules/.bin/openapi-generator-cli generate \
		-i ./openapi.yml \
		-g rust-axum \
		-o ./backend/application/generated \
		-p packageName=generated
