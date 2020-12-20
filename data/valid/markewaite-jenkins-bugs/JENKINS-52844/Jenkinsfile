pipeline {
  agent none
  options {
    skipDefaultCheckout(true)
    buildDiscarder(logRotator(artifactDaysToKeepStr: '2', artifactNumToKeepStr: '5', daysToKeepStr: '15', numToKeepStr: '15'))
    durabilityHint('PERFORMANCE_OPTIMIZED')
  }
  environment {
    name='ビルド番号をインクリメント and “Ω” should be a greek uppercase omega letter enclosed in quotation marks.' // Japanese text
  }
  stages {
    stage('Unix echo non-English text') {
      agent {
        label '!windows && git-1.9+'
      }
      steps {
	checkout([$class: 'GitSCM',
		  branches: [[name: 'JENKINS-52844']],
		  extensions: [
                                [$class: 'CloneOption', honorRefspec: true, noTags: true, reference: '/var/lib/git/mwaite/bugs/jenkins-bugs.git', shallow: true, depth: 1, timeout: 3],
                                [$class: 'LocalBranch', localBranch: 'JENKINS-52844']
                              ],
		  gitTool: scm.gitTool,
		  userRemoteConfigs: scm.userRemoteConfigs,
                 ])
        echo "Environment name is ${env.name}"
        withAnt(installation: 'ant-latest') {
          sh 'ant info'
        }
      }
    }
    stage('Windows echo non-English text') {
      agent {
        label 'windows && git-1.9+'
      }
      steps {
	checkout([$class: 'GitSCM',
		  branches: [[name: 'JENKINS-52844']],
		  extensions: [
                                [$class: 'CloneOption', honorRefspec: true, noTags: true, reference: '/var/lib/git/mwaite/bugs/jenkins-bugs.git', shallow: true, depth: 1, timeout: 3],
                                [$class: 'LocalBranch', localBranch: 'JENKINS-52844']
                              ],
		  gitTool: scm.gitTool,
		  userRemoteConfigs: scm.userRemoteConfigs,
                 ])
        echo "Environment name is ${env.name}"
        withAnt(installation: 'ant-latest') {
          bat 'ant info'
        }
      }
    }
  }
}
