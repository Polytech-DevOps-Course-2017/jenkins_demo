# Meaningless app in Rust #

## Quick start ##

### Simple way ###

To run with configured jenkins:

```shell
cd meaningless-app
make run-heavy
```

This will:
* download the pre-configured `JENKINS_HOME` folder
  from [qezz/jenkins-heavy-var](https://github.com/qezz/jenkins-heavy-var.git).
* run docker container with jenkins on port **8082**

**Important**

`jenkins-heavy-var` is going to be the hude submodule, so do not try
to include it's content to this (almost lightweight) repo.

### DIY way ###

You can bootstrap your own JENKINS_HOME and use provided Jenkinsfile to build a pipeline.

See Makefile to properly attach your *jenkins_home* to the docker container.

## Things ##

### Pipeline ###

Features:

* Build every 1h
* 8 stages
  1. checkout the repo
  2. prepare
  3. build (debug)
  4. test (debug)
  5. build (release)
  6. test (release)
  7. generate docs
  8. run the release app
* configured for `guest` user (see below)

### Monitoring Plugin ###

See JavaMelody Monitoring at http://localhost:8082/monitoring

### Matrix Authorization Strategy Plugin ###

Users:
* admin:admin
* guest:guest (read only for one particular pipeline 'rust-simple' )

![images](https://qezz.github.io/shared/jenkins-users.png "Hey")
![meow](https://qezz.github.io/shared/clojure-website-heroku.png "Privet")


## License ##

Beer-ware or WTFPL

## Author ##

Sergey Mishin, sergei.a.mishin@gmail.com
