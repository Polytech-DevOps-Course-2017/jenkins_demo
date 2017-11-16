all: build

build:
	docker build -t qezz/pystorage-jenkins:0.1 .

wo-cache:
	docker build --no-cache -t qezz/pystorage-jenkins:0.1 .

push:
	docker push qezz/pystorage-jenkins:0.1
