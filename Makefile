.PHONY: build up down logs restart ps

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

ps:
	docker compose ps