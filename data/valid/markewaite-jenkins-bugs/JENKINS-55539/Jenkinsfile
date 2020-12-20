#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
    agent {
        label '!windows'
    }

    options {
        checkoutToSubdirectory('This-is-a-directory')
    }

    stages {
        stage("Build") {
            steps {
                withAnt(installation: "ant-latest", jdk: "jdk8") {
                    sh "ant -f This-is-a-directory/build.xml info"
                }
                logContains([expectedRegEx: ".*Buildfile:.*This-is-a-directory.*",
                             failureMessage: "Missing expected build file name"])
            }
        }
    }
}
