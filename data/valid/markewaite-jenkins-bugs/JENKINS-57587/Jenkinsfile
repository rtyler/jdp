#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
    agent {
        label '!windows'
    }

    stages {
        stage("Build") {
            steps {
                withAnt(installation: 'ant-latest', jdk: 'jdk8') {
                    sh 'ant info'
                }
                logContains([expectedRegEx: '.*java is.*',
                             failureMessage: 'Missing expected java version report'])
            }
        }
    }
}
