#! /bin/bash

docker run \
	--name postgres \
	-e POSTGRES_USER=cookingd \
	-e POSTGRES_PASSWORD=1234 \
	-e POSTGRES_DB=cookingd \
	-p 5432:5432 \
	-d postgres:alpine