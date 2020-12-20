#!/usr/bin/env groovy

pipeline {
    agent { label 'docker' }
    stages {
        stage('Prepare') {
            steps {
                sh 'make depends'
            }
        }
        stage('Build') {
            steps {
                sh 'make container'
            }
        }
        stage('Test') {
            steps {
                sh 'make check'
            }
        }
        stage('Documentation') {
            steps {
                sh 'make documentation'
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'reports/*.html', fingerprint: true
        }
        success {
            archiveArtifacts artifacts: 'doc/**', fingerprint: true
        }
    }
}
