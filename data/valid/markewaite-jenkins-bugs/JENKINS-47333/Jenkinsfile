pipeline {
    agent {
        label '!windows'
    }

    parameters {
        file description: 'Uploaded file parameter to test JENKINS-47333', name: 'test-JENKINS-47333'
    }

    stages {
        stage("List workspace contents") {
            steps {
                sh 'ls'
            }
        }
    }
}
