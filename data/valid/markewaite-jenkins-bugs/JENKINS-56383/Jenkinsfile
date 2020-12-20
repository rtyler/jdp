pipeline {
    agent {
        label '!windows'
    }
    options {
        skipDefaultCheckout(true)
        buildDiscarder(logRotator(artifactDaysToKeepStr: '2', artifactNumToKeepStr: '5', daysToKeepStr: '15', numToKeepStr: '15'))
    }
    tools {
        ant 'ant-latest'
    }
    stages {
        stage('Checkout') {
            steps {
                checkout(poll: true,
                         scm: [$class: 'GitSCM',
                               branches: [[name: 'JENKINS-56383']],
                               extensions: [
                                            [$class: 'CheckoutOption', timeout: 3],
                                            [$class: 'CloneOption', honorRefspec: true, noTags: true, reference: '/var/lib/git/mwaite/bugs/jenkins-bugs.git'],
                                            [$class: 'LocalBranch', localBranch: 'JENKINS-56383'],
                                           ],
                               gitTool: scm.gitTool,
                               userRemoteConfigs: scm.userRemoteConfigs])
            }
        }
        stage('Build') {
	    parallel {
		stage('Build Up') {
		    steps {
			sh 'ant info'
		    }
		}
		stage('Build Down') {
		    steps {
			sh 'echo build down'
		    }
		}
	    }
        }
        stage('Test') {
	    parallel {
		stage('Test Up') {
		    steps {
			sh 'echo test up'
		    }
		}
		stage('Test Down') {
		    steps {
			sh 'echo test down'
		    }
		}
	    }
        }
        stage('Deploy') {
            steps {
                sh 'echo deploy'
            }
        }
    }
    post {
        always {
            /* Confirmed that if deleteDir is there, then multibranch pipeline will build the branch on every poll. */
            /* Confirmed that without deleteDir, then multibranch pipeline will not build the branch on every poll. */
            deleteDir()
        }
    }
}
