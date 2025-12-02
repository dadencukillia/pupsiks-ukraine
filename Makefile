build_production:
	docker compose build

build_test:
	docker compose -f docker-compose.test.yml build

production:
	docker compose up --build -d

test: stop
	docker compose -f docker-compose.test.yml up --build --attach tests --abort-on-container-failure --exit-code-from tests
	docker compose down

stop:
	docker compose down

logs:
	docker compose logs --follow
