#!groovy

@Library('globalPipelineLibraryMarkEWaite') _

pipeline {
    agent {
        label '!windows && git-1.9+' // Shallow checkout fails on git before 1.9
    }

    options {
        skipDefaultCheckout()
    }

    stages {
        stage("root build.xml") {
            steps {
                // Initial checkout to provide only build.xml at root of workspace
                checkout([$class: 'GitSCM',
                          branches: [
                              [name: 'JENKINS-60617']
                          ],
                          extensions: [
                              [$class: 'CloneOption', depth: 1, honorRefspec: true, noTags: true, reference: '/var/lib/git/mwaite/bugs/jenkins-bugs.git', shallow: true],
                              [$class: 'SparseCheckoutPaths', sparseCheckoutPaths: [[path: 'build.xml']]], // exactly one file we need, nothing else
                          ],
                          userRemoteConfigs: [
                              [refspec: '+refs/heads/JENKINS-60617:refs/remotes/origin/JENKINS-60617', url: 'https://github.com/MarkEWaite/jenkins-bugs']
                          ]
                         ]
                )
            }
        }
        stage("Parallel Subdirs") {
            parallel {
                stage('Defaults: git plugin') {
                    steps {
                        dir('git-step-with-defaults') {
                            git 'https://github.com/jenkinsci/git-plugin'
                            withAnt(installation: 'ant-latest', jdk: 'jdk8') {
                                sh 'ant -f ../build.xml info'
                            }
                            logContains([expectedRegEx: '.*echo.*user dir is.*git-step-with-defaults.*',
                                         failureMessage: 'Missing expected subdirectory git-step-with-defaults'])
                            logContains([expectedRegEx: '.*echo.*git origin url .*git-step-with-defaults.* is https://github.com/jenkinsci/git-plugin',
                                         failureMessage: 'Missing expected origin url git-plugin for git-step-with-defaults'])
                        }
                    }
                }
                stage('Branch: git client plugin') {
                    steps {
                        dir('git-step-with-https-and-branch') {
                            git branch: 'stable-2.x',
                                url: 'https://github.com/jenkinsci/git-client-plugin.git'
                            withAnt(installation: 'ant-latest', jdk: 'jdk8') {
                                sh 'ant -f ../build.xml info'
                            }
                            logContains([expectedRegEx: '.*echo.*user dir is.*git-step-with-https-and-branch.*',
                                         failureMessage: 'Missing expected subdirectory git-step-with-https-and-branch'])
                            logContains([expectedRegEx: '.*echo.*git origin url .*git-step-with-https-and-branch.* is https://github.com/jenkinsci/git-client-plugin.git',
                                         failureMessage: 'Missing expected origin url git-client-plugin.git for git-step-with-https-and-branch'])
                        }
                    }
                }
                stage('Credentials: git client plugin') {
                    steps {
                        dir('git-step-with-ssh-and-credential') {
                            git credentialsId: 'MarkEWaite-github-rsa-private-key',
                                url: 'git@github.com:jenkinsci/git-client-plugin.git'
                            withAnt(installation: 'ant-latest', jdk: 'jdk8') {
                                sh 'ant -f ../build.xml info'
                            }
                            logContains([expectedRegEx: '.*echo.*user dir is.*git-step-with-ssh-and-credential.*',
                                         failureMessage: 'Missing expected subdirectory git-step-with-ssh-and-credential'])
                            logContains([expectedRegEx: '.*echo.*git origin url .*git-step-with-ssh-and-credential.* is git@github.com:jenkinsci/git-client-plugin.git',
                                         failureMessage: 'Missing expected origin url git-client-plugin.git for git-step-with-ssh-and-credential'])
                        }
                    }
                }
                stage('No changelog: credentials plugin') {
                    steps {
                        dir('git-step-with-https-and-changelog') {
                            git changelog: false,
                                url: 'https://github.com/jenkinsci/credentials-plugin.git'
                            withAnt(installation: 'ant-latest', jdk: 'jdk8') {
                                sh 'ant -f ../build.xml info'
                            }
                            logContains([expectedRegEx: '.*echo.*user dir is.*git-step-with-https-and-changelog.*',
                                         failureMessage: 'Missing expected subdirectory git-step-with-https-and-changelog'])
                            logContains([expectedRegEx: '.*echo.*git origin url .*git-step-with-https-and-changelog.* is https://github.com/jenkinsci/credentials-plugin.git',
                                         failureMessage: 'Missing expected origin url credentials-plugin.git for git-step-with-https-and-changelog'])
                        }
                    }
                }
                stage('No poll:platform labeler plugin') {
                    steps {
                        dir('git-step-with-git-and-polling') {
                            git poll: false,
                                url: 'git://github.com/jenkinsci/platformlabeler-plugin.git'
                            withAnt(installation: 'ant-latest', jdk: 'jdk8') {
                                sh 'ant -f ../build.xml info'
                            }
                            logContains([expectedRegEx: '.*echo.*user dir is.*git-step-with-git-and-polling.*',
                                         failureMessage: 'Missing expected subdirectory git-step-with-git-and-polling'])
                            logContains([expectedRegEx: '.*echo.*git origin url .*git-step-with-git-and-polling.* is git://github.com/jenkinsci/platformlabeler-plugin.git',
                                         failureMessage: 'Missing expected origin url platformlabeler-plugin.git for git-step-with-git-and-polling'])
                        }
                    }
                }
            }
        }
    }
}
