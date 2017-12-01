CURRENTWD = $(shell pwd)

all:
	echo "Select a target"

build:
	docker build -t qezz/meaningless-app:0.1 .

wo-cache:
	docker build --no-cache -t qezz/meaningless-app:0.1 .

push:
	docker push qezz/meaningless-app:0.1

jenkins-heavy-var:
	$(shell if [ -d docker_demo ]; then; echo "exists"; else; git clone https://github.com/qezz/jenkins-heavy-var.git ../jenkins-heavy-var fi)


run-heavy: jenkins-heavy-var
	echo $(CURRENTWD)
	docker run -p 8082:8080 -p 50000:50000 -v "$(CURRENTWD)/../jenkins-heavy-var/jenkins_var":/var/jenkins_home qezz/meaningless-app:0.1

clean:
	rm -rf ./jenkins-heavy-var
