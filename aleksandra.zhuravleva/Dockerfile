FROM jenkins:latest

LABEL maintainer "Aleksandra Zhuravleva <guravleva.aleksandra@gmail.com>"

USER root

RUN apt-get update
RUN apt-get install -y maven

ENTRYPOINT ["/bin/tini", "--", "/usr/local/bin/jenkins.sh"]
