# Rest Service build by Jenkins

## Description
The repository contains Jenkinsfile pipeline which you can use to
build 2 projects.

## Quick start
You can run pre-configured jenkins with `make run-full` or use
lightweight version (without baked `/var/jenkins_home` directory right
into the container) with `make run-light`. When latter, you should
provide your own `jenkins_home` directory into the local
`var_jenkins_home`.

## Docker images

Docker images are provided in two types:
* [Full](https://hub.docker.com/r/sashadock/jenkins_full/): ready-to-use container with `/var/jenkins_home` baked into
* [Light](https://hub.docker.com/r/sashadock/jenkins_demo/): without `/var/jenkins_home`

## Features

### Pipeline

Pipeline consists of 4 stages:
1. Preparation
  * clean up 
  * and checkout the repos
2. Build Hibernate
  * build `hibernate-db-analyzer` package with maven
3. Build Spring
  * build `SpringDBAnalyzer` package with maven
4. Test
  * Run tests for both applications
5. Deploy
  * For now, it just creates 'docker-compose.yml' file
  
See Jenkinsfile for detailed pipeline

### User management

There are two users:
* admin:admin (Sasha) - can do anything
* guest:guest (Anonymous Guest) - can only 'Read' the Job

### Special settings 

* There is a possibility to choose JDK version 7 or 8 before the build.
* `archive 'build/libs/**/*.jar'` is used to grab built artifacts.
* The job is built every 15 minutes.

### Monitoring

'Java Melody node monitoring' was installed as a monitoring tool.
Check the http://localhost:8080/monitoring out.

## Author
Aleksandra Zhuravleva, guravleva.aleksandra@gmail.com
