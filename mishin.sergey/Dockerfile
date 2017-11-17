FROM jenkins:latest

LABEL maintainer="Sergey Mishin <sergei.a.mishin@gmail.com>"

RUN ls -la /home

USER root

RUN curl -L https://github.com/docker/compose/releases/download/1.17.1/docker-compose-`uname -s`-`uname -m` -o /usr/local/bin/docker-compose
RUN chmod +x /usr/local/bin/docker-compose
RUN docker-compose --version

RUN mkdir -p /tmp/scripts/pystorage
COPY Jenkinsfile /tmp/scripts/pystorage/Jenkinsfile

ENTRYPOINT ["/bin/tini", "--", "/usr/local/bin/jenkins.sh"]
