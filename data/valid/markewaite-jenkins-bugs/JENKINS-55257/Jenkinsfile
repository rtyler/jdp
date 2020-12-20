pipeline {
    options {
        timestamps()
        skipDefaultCheckout()
    }
    agent {
        label 'windows'
    }
    stages {
        stage('Info') {
            steps {
                // git 'https://github.com/amuniz/maven-helloworld'
                // git branch: 'JENKINS-55257', url: 'https://github.com/MarkEWaite/jenkins-bugs'
                checkout([
                  $class: 'GitSCM',
                  branches: scm.branches,
                  extensions: scm.extensions,
                  userRemoteConfigs: scm.userRemoteConfigs
                ])
                withAnt(installation: 'ant-latest') {
                    bat 'ant info'
                }
            }
        }
    }
}
