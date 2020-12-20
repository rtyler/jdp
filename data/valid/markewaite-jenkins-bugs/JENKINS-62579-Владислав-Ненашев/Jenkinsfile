#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
  agent {
    label 'windows'
  }
  tools {
    ant 'ant-latest'
  }
  options {
    skipDefaultCheckout true // Reasonable that SCM vars not defined, since no default checkout?
  }
  stages {
    stage('Build') {
      steps {
        echo "Branches[0] is ${scm.branches[0]}"
        script {
          scmVars = checkout([$class: 'GitSCM',
                            branches: scm.branches,
                            extensions: [[$class: 'CloneOption', honorRefspec: true, noTags: true, reference: '/var/lib/git/mwaite/bugs/jenkins-bugs.git']],
                            gitTool: scm.gitTool,
                            userRemoteConfigs: [[url: 'ssh://git@github.com/MarkEWaite/jenkins-bugs.git',
                                                 credentialsId: 'MarkEWaite-github-rsa-private-key-has-passphrase',
                                                 refspec: '+refs/heads/JENKINS-62579-Владислав-Ненашев:refs/remotes/origin/JENKINS-62579-Владислав-Ненашев']]])
          bat 'ant info'
          logContains(expectedRegEx: ".*Git HEAD is ${scmVars.GIT_COMMIT}.*",
                      failureMessage: "Missing checkout return of GIT_COMMIT value '${scmVars.GIT_COMMIT}'")
        }
        // Reasonable that env.GIT_COMMIT is not set, since there was no default checkout
        logDoesNotContain(expectedRegEx: ".*Git HEAD is ${env.GIT_COMMIT}.*",
                          failureMessage: "Missing env GIT_COMMIT value '${env.GIT_COMMIT}'")
      }
    }
  }
}
