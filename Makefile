.PHONY: generate, curl-get-recent, curl-get-all, curl-put

generate:
	./node_modules/.bin/openapi-generator-cli generate \
		-i ./openapi.yml \
		-g rust-axum \
		-o ./backend/application/generated \
		-p packageName=generated
curl-get-recent:
	curl --dump-header - \
	localhost:8080/get/recent
curl-get-all:
	curl --dump-header - \
	localhost:8080/get/all
curl-put:
	curl --dump-header - \
	-X PUT \
	-H "Content-Type: application/json" \
	-d '{ "vocabulary": { "spelling": "run", "meaning": "moved at a speed faster than a walk", "part_of_speech": "verb" } }' \
	localhost:8080/register
