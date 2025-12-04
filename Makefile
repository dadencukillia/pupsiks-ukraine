build_production:
	docker compose build

build_production_https:
	docker compose -f docker-compose.yml -f docker-compose.https.yml build

build_test:
	docker compose -f docker-compose.test.yml build

production:
	docker compose up -d

production_https:
	docker compose -f docker-compose.yml -f docker-compose.https.yml up -d

test: stop
	docker compose -f docker-compose.test.yml up --build --attach tests --abort-on-container-failure --exit-code-from tests --force-recreate
	docker compose down

stop:
	docker compose down

logs:
	docker compose logs --follow

first_ssl_certs:
	docker compose -f docker-compose.yml -f docker-compose.https.yml run -p "80:80" --rm certbot -c "certbot certonly --standalone -d \$$SERVER_NAME --non-interactive --agree-tos -m \$$SSL_OWNER_EMAIL"

renew_ssl_certs:
	docker compose -f docker-compose.yml -f docker-compose.https.yml run --rm certbot -c "certbot renew -v --deploy-hook 'echo CERT_IS_UPDATES_CODE' | grep CERT_IS_UPDATES_CODE -q" && \
		docker compose exec nginx sh -c "nginx -s reload"
