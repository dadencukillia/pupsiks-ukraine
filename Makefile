.PHONY: build_production build_production_https build_test production production_https test stop logs first_ssl_certs renew_ssl_certs dev

PUPSIKS_CONTAINER_ENGINE ?= docker # or "podman"

build_production:
	$(PUPSIKS_CONTAINER_ENGINE) compose build

build_production_https:
	$(PUPSIKS_CONTAINER_ENGINE) compose -f docker-compose.yml -f docker-compose.https.yml build

build_test:
	$(PUPSIKS_CONTAINER_ENGINE) compose -f docker-compose.test.yml build

production:
	$(PUPSIKS_CONTAINER_ENGINE) compose up -d

production_https:
	$(PUPSIKS_CONTAINER_ENGINE) compose -f docker-compose.yml -f docker-compose.https.yml up -d

test: stop
	$(PUPSIKS_CONTAINER_ENGINE) compose -f docker-compose.test.yml up --build --attach tests --abort-on-container-failure --exit-code-from tests --force-recreate
	$(PUPSIKS_CONTAINER_ENGINE) compose down

stop:
	$(PUPSIKS_CONTAINER_ENGINE) compose down

logs:
	$(PUPSIKS_CONTAINER_ENGINE) compose logs --follow

first_ssl_certs:
	$(PUPSIKS_CONTAINER_ENGINE) compose -f docker-compose.yml -f docker-compose.https.yml run -p "80:80" --rm certbot -c "certbot certonly --standalone -d \$$SERVER_NAME --non-interactive --agree-tos -m \$$SSL_OWNER_EMAIL"

renew_ssl_certs:
	$(PUPSIKS_CONTAINER_ENGINE) compose -f docker-compose.yml -f docker-compose.https.yml run --rm certbot -c "certbot renew -v --deploy-hook 'echo CERT_IS_UPDATES_CODE' | grep CERT_IS_UPDATES_CODE -q" && \
		$(MAKE) restart_nginx

dev:
	$(PUPSIKS_CONTAINER_ENGINE) compose -f docker-compose.dev.yml up --build

restart_nginx:
	$(PUPSIKS_CONTAINER_ENGINE) compose exec nginx sh -c "nginx -s reload"
