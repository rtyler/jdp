#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
  agent {
    label '!windows && !cloud && linux' // Need http access to Jenkins server and a /bin/bash program
  }
  tools {
    ant 'ant-latest'
  }
  options {
    skipDefaultCheckout(true)
  }
  stages {
    stage('Checkout') {
      steps {
        deleteDir()
        checkout([$class: 'GitSCM',
                  branches: scm.branches,
                  extensions: [[$class: 'CloneOption', honorRefspec: true, noTags: true, reference: '/var/lib/git/mwaite/bugs/jenkins-bugs.git']],
                  gitTool: 'Default', // Command line git only
                  userRemoteConfigs: scm.userRemoteConfigs
                ])
        sh 'ant info'
        logContains(expectedRegEx: ".*Count of git fetch on agent: 1.*",
                    failureMessage: "Wrong git fetch count in declarative pipeline")
      }
    }
  }
}
