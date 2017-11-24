# Jenkins: java builder

## Description

It's a simple Pipeline script for Jenkins for building Java application with parameters. There are 2 version for linux and windows.

## Quick start with docker

1. `docker pull mrsandmanrus/custom_jenkins`
2. `docker run -p 8080:8080 -p 50000:50000 -p 8888:8888 mrsandmanrus/custom_jenkins`
3. Go to: 127.0.0.1:8080
4. Log as admin/admin
5. Start Java_builder with default parameters
6. Go to: 127.0.0.1:8080

##Additional info

Login/password:
* admin/admin
* user/user (read only)

Custom plugins:
* Build Monitor Plugin
* Role-based Authorization Strategy
