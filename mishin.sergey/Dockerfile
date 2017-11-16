FROM jenkins:latest

LABEL maintainer="Sergey Mishin <sergei.a.mishin@gmail.com>"

RUN ls -la /home

RUN mkdir -p /tmp/scripts/pystorage
COPY Jenkinsfile /tmp/scripts/pystorage/Jenkinsfile

ENTRYPOINT ["/bin/tini", "--", "/usr/local/bin/jenkins.sh"]
