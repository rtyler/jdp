#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
    agent {
        label '!windows'
    }
    tools {
        ant 'ant-latest'
    }
    options {
        skipDefaultCheckout(true)
    }

    stages {
        stage("Build") {
            steps {
                checkout(poll: true,
                         scm: [$class: 'GitSCM',
                               branches: [[name: 'JENKINS-33202']],
                               extensions: [
                                            [$class: 'CloneOption', honorRefspec: true, noTags: true, reference: '/var/lib/git/mwaite/bugs/jenkins-bugs.git'],
                                            [$class: 'LocalBranch', localBranch: '**'],
                                           ],
                               gitTool: scm.gitTool,
                               userRemoteConfigs: scm.userRemoteConfigs])
                sh 'ant info'
                logContains([expectedRegEx: '.*java is.*',
                             failureMessage: 'Missing expected java version report'])
            }
        }
    }
}
