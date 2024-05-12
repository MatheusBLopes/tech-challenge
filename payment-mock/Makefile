PROJECT_NAME = $(shell pwd | rev | cut -f1 -d'/' - | rev)

runserver:
	poetry run python payment-api/manage.py runserver


lint:
	poetry run pre-commit run --all-files

run-gunicorn:
	gunicorn -c gunicorn_config.py payment_api.wsgi:application

# DOCKER
run-container:
	docker run -p 9000:9000 payment_api

build:
	docker build -t payment_api .

