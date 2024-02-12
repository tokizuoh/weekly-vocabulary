.PHONY: generate, curl-get-recent, curl-get-all, curl-put

generate:
	./node_modules/.bin/openapi-generator-cli generate \
		-i ./openapi.yml \
		-g rust-axum \
		-o ./backend/application/generated \
		-p packageName=generated
curl-get-vocabulary-recent:
	curl --dump-header - \
	localhost:8080/vocabulary/recent
curl-get-vocabulary-all:
	curl --dump-header - \
	localhost:8080/vocabulary/all
curl-post-vocabulary:
	curl --dump-header - \
	-X POST \
	-H "Content-Type: application/json" \
	-d '{ "vocabulary": { "spelling": "run", "meaning": "moved at a speed faster than a walk", "part_of_speech": "verb" } }' \
	localhost:8080/vocabulary
curl-put-vocabulary:
	curl --dump-header - \
	-X PUT \
	-H "Content-Type: application/json" \
	-d '{ "vocabulary": { "id": $(ID), "spelling": "CHANGED-SPELLING", "meaning": "CHANGED-MEANING", "part_of_speech": "interjection" } }' \
	localhost:8080/vocabulary
curl-delete-vocabulary-id:
	curl --dump-header - \
	-X DELETE \
	-H "Content-Type: application/json" \
	localhost:8080/vocabulary/${ID}
