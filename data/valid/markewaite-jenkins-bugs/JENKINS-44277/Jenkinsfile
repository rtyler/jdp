#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
    agent {
        label '!windows'
    }
    options {
        checkoutToSubdirectory('test-subdirectory')
        quietPeriod(29)
    }

    stages {
        stage("Build") {
            options {
                timeout(time: 7, unit: 'MINUTES')
                timestamps()
            }
            steps {
                withAnt(installation: 'ant-latest', jdk: 'jdk8') {
                    sh 'ant -f test-subdirectory/build.xml info'
                }
                logContains([expectedRegEx: '.*echo. base dir is .*test-subdirectory.*',
                             failureMessage: 'Missing expected test-subdirectory contents'])
                logContains([expectedRegEx: '.*echo. test-subdirectory contains .*build.xml.*',
                             failureMessage: 'Missing expected test-subdirectory contents'])
            }
        }
    }
}
