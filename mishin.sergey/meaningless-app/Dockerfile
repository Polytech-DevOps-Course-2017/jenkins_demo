FROM jenkins:latest

LABEL maintainer="Sergey Mishin <sergei.a.mishin@gmail.com>"

RUN ls -la /home

USER root

# install rust
RUN curl https://sh.rustup.rs -sSf -o rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh -y

# Copy Jenkinsfile
RUN mkdir -p /tmp/scripts/pystorage
COPY Jenkinsfile /tmp/scripts/pystorage/Jenkinsfile

ENTRYPOINT ["/bin/tini", "--", "/usr/local/bin/jenkins.sh"]
