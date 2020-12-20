#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
  agent {
    label '!windows && !cloud && linux' // Need http access to Jenkins server and a /bin/bash program
  }
  tools {
    ant 'ant-latest'
  }
  stages {
    stage('Count fetch statements on agent') {
      steps {
        sh 'ant info'
        deleteDir() // Require full clone on next checkout
        logContains(expectedRegEx: ".*Count of git fetch on agent: 1.*",
                    failureMessage: "Wrong git fetch count in declarative pipeline")
      }
    }
  }
}
