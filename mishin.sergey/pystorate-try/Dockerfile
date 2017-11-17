FROM jenkins:latest

LABEL maintainer="Sergey Mishin <sergei.a.mishin@gmail.com>"

RUN ls -la /home

USER root

# install... Docker
RUN apt-get update
RUN apt-get install -y \
         apt-transport-https \
         ca-certificates \
         curl \
         software-properties-common
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add -
RUN apt-key fingerprint 0EBFCD88
RUN add-apt-repository \
    "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) \
    stable"
RUN apt-get update
RUN apt-get install -y docker-ce
    
# install docker-compose
RUN curl -L https://github.com/docker/compose/releases/download/1.17.1/docker-compose-`uname -s`-`uname -m` -o /usr/local/bin/docker-compose
RUN chmod +x /usr/local/bin/docker-compose
RUN docker-compose --version

# Copy Jenkinsfile
RUN mkdir -p /tmp/scripts/pystorage
COPY Jenkinsfile /tmp/scripts/pystorage/Jenkinsfile

ENTRYPOINT ["/bin/tini", "--", "/usr/local/bin/jenkins.sh"]
