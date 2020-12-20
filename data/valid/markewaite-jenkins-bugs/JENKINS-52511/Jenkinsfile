pipeline {
  agent none
  options {
    durabilityHint('PERFORMANCE_OPTIMIZED')
  }
  stages {
    stage('parallel') {
      parallel {
        stage('windows') {
          agent {
            label 'windows'
          }
          steps {
            echo "pipeline GIT_COMMIT is ${env.GIT_COMMIT}"

            script {
              // extract author name, committer name, and commit message of GIT_COMMIT
              def author_name = bat(script: "@echo off\ngit log -n 1 ${env.GIT_COMMIT} --format=%%aN", returnStdout: true).trim()
              echo "Author name of last commit is ${author_name}"
              def committer_name = bat(script: "@echo off\ngit log -n 1 ${env.GIT_COMMIT} --format=%%cN", returnStdout: true).trim()
              echo "Committer name of last commit is ${committer_name}"
              def commit_message = powershell(script: "git log -1 --format=%B ${GIT_COMMIT}", returnStdout: true).trim()
              echo "Commit message of last commit is ${commit_message}"
            }

            ws(dir: WORKSPACE + '/windows-dir') {
              echo "pipeline GIT_AUTHOR_NAME in windows ws is ${env.GIT_AUTHOR_NAME}"
              bat "echo bat GIT_COMMITTER_NAME in windows ws is %GIT_COMMITTER_NAME%"
            }
            echo "pipeline GIT_COMMIT after windows ws is ${env.GIT_COMMIT}"
            bat "echo bat GIT_COMMIT after windows ws is %GIT_COMMIT%"
            bat 'echo hello windows from %COMPUTERNAME%'
            withAnt(installation: 'ant-latest') {
              bat 'ant info'
            }
          }
        }
        stage('linux') {
          agent {
            label 'linux'
          }
          steps {
            echo "pipeline GIT_COMMIT before linux ws is ${env.GIT_COMMIT}"

            script {
              // extract author name, committer name, and commit message of GIT_COMMIT
              def author_name = sh(script: "git log -n 1 ${env.GIT_COMMIT} --format=%aN", returnStdout: true).trim()
              echo "Author name of last commit is ${author_name}"
              def committer_name = sh(script: "git log -n 1 ${env.GIT_COMMIT} --format=%cN", returnStdout: true).trim()
              echo "Committer name of last commit is ${committer_name}"
              def commit_message = sh(script: "git log -1 --format=%B ${GIT_COMMIT}", returnStdout: true).trim()
              echo "Commit message of last commit is ${commit_message}"
            }

            ws(dir: WORKSPACE + '/linux-dir') {
              echo "pipeline GIT_AUTHOR_NAME in linux ws is ${env.GIT_AUTHOR_NAME}"
              sh "echo sh GIT_COMMITTER_NAME in linux ws is $GIT_COMMITTER_NAME"
            }
            echo "pipeline GIT_COMMIT after linux ws is ${env.GIT_COMMIT}"
            sh "echo sh GIT_COMMIT after linux ws is $GIT_COMMIT"
            echo 'Workspace after linux ws is ' + WORKSPACE
            sh 'echo hello linux from `hostname`'
            withAnt(installation: 'ant-latest') {
              sh 'ant info'
            }
          }
        }
      }
    }
  }
}
