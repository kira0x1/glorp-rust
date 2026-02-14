pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh "echo \"kira0x1/glorp-rust:${env.BUILD_NUMBER}\""
                sh "docker build -t \"kira0x1/glorp-rust:${env.BUILD_NUMBER}\" ."
            }
        }
        stage('Push') {
            steps {
                sh "docker image ls"
                // sh "docker image push \"kira0x1/rustci-test:${env.BUILD_NUMBER}\""
            }
        }
    }
}