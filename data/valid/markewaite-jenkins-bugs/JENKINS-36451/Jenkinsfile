pipeline {
    agent {
        label '!windows'
    }

    options {
        durabilityHint('PERFORMANCE_OPTIMIZED')
    }

    parameters {
        booleanParam(defaultValue: true, description: 'Job will fail if this is not checked', name: 'booleanParamJENKINS36451')
    }

    stages {
        stage("pass if param is true") {
            when {
                expression { return params.booleanParamJENKINS36451 }
            }
            steps {
                echo "stage will pass due to ${params.booleanParamJENKINS36451} parameter"
            }
        }
        stage("fail if param is false") {
            when {
                expression { return ! params.booleanParamJENKINS36451 }
            }
            steps {
                echo "stage will fail due to ${params.booleanParamJENKINS36451} parameter"
                sh 'exit 1'
            }
        }
    }
}
