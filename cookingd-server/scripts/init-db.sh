#! /bin/bash

docker run \
	--name postgres \
	-e POSTGRES_USER=postgres \
	-e POSTGRES_PASSWORD=password \
	-e POSTGRES_DB=cookingd \
	-p 5432:5432 \
	-d postgres:alpine