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
            echo 'Workspace before windows ws is ' + WORKSPACE
            ws(dir: WORKSPACE + '/windows-dir') {
              echo 'Workspace inside windows ws is ' + WORKSPACE
            }
            echo 'Workspace after ws windows is ' + WORKSPACE
            bat 'echo hello windows from %COMPUTERNAME%'
            bat 'if not exist build.xml exit 1'
          }
        }
        stage('linux') {
          agent {
            label 'linux'
          }
          steps {
            echo 'Workspace before linux ws is ' + WORKSPACE
            ws(dir: WORKSPACE + '/linux-dir') {
              echo 'Workspace inside linux ws is ' + WORKSPACE
            }
            echo 'Workspace after linux ws is ' + WORKSPACE
            sh 'echo hello linux from `hostname`'
            sh '[ -f build.xml ] || exit 1'
          }
        }
      }
    }
  }
}
