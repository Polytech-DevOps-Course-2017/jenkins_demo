build-light:
	docker build --no-cache -t sashadock/jenkins_demo .

build-full:
	docker build --no-cache -f Dockerfile_full -t sashadock/jenkins_full .

run-light:
	docker run -p 8080:8080 -p 50000:50000 -v $(CURDIR)/var_jenkins_home:/var/jenkins_home sashadock/jenkins_demo
	
run-full:
	docker run -p 8080:8080 -p 50000:50000 sashadock/jenkins_full

