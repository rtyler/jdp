#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
  agent {
    label 'windows' // For bat build step
  }
  tools {
    ant 'ant-latest'
    git 'git-windows'
  }
  stages {
    stage('Build') {
      steps {
        bat 'ant info'
        logContains(expectedRegEx: ".*Git HEAD is ${env.GIT_COMMIT}.*",
                    failureMessage: "Missing env GIT_COMMIT value '${env.GIT_COMMIT}'")
      }
    }
  }
}
