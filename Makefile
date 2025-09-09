.PHONY: build up down logs restart

build:
	docker compose build

up:
	docker compose up --build --force-recreate -d

down:
	docker compose down

logs:
	docker compose logs -f

restart:
	docker compose restart
