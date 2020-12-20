#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
  agent any
  tools {
    ant 'ant-latest'
  }
  stages {
    stage('Check refspec in fetch') {
      steps {
        script {
          withAnt(installation: 'ant-latest') {
            if (isUnix()) {
              sh 'ant info'
            } else {
              bat 'ant info'
            }
          }
        }
        deleteDir() // Require full clone on next checkout
        logContains(expectedRegEx: '.*.exec. [+]refs/heads/JENKINS-56063-refspec-env-reference-not-expanded:refs/remotes/origin/JENKINS-56063-refspec-env-reference-not-expanded$',
                    failureMessage: 'Expected remote.origin.fetch not found in output')
        logDoesNotContain(expectedRegEx: '.*[+]refs/heads/.*JOB_BASE_NAME.:refs/remotes/origin/.*JOB_BASE_NAME..*',
                    failureMessage: 'Unexpected JOB_BASE_NAME found in output')
      }
    }
  }
}
