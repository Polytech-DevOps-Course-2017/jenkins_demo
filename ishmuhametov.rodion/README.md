# Jenkins in a Docker Container
`Dockerfile` puts Jenkins in a Docker Container with ability to retrieve logs. `Jenkinsfile` runs a Maven build and loads up results using the JUnit plugin.

## Commands
* `docker pull irodionr/jenkins-test` - get repository
* `docker run -p 8080:8080 --name=jenkins-test1 -d jenkins-test` - start Jenkins
* `docker-machine ip` - get IP of the Docker machine
* Go to ip:8080 to access Jenkins
* `docker exec jenkins-test1 tail -f /var/log/jenkins/jenkins.log` - tail the log file
* `docker cp jenkins-test1:/var/log/jenkins/jenkins.log jenkins.log` + `cat jenkins.log` - retrieve logs if Jenkins crashes
  
## Screenshot
![Dashboard](https://i.imgur.com/xYdG6Kb.png)
Admin and user accounts
