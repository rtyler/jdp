#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
    agent {
        label '!windows'
    }
    tools {
        ant 'ant-latest'
        git 'jgit'
    }

    stages {
        stage("Build") {
            steps {
                sh 'ant info'
                logContains([expectedRegEx: '.*java is.*',
                             failureMessage: 'Missing expected java version report'])
                logDoesNotContain([expectedRegEx: '.*> git fetch .*',
                             failureMessage: 'Used command line git instead of JGit'])
            }
        }
    }
}
