pipeline {
    agent any
    environment {
        IMAGE_NAME = "kira0x1/glorp-rust"
    }
    stages {
        stage('Build') {
            steps {
                sh """
                    docker build \
                      -t ${IMAGE_NAME}:${BUILD_NUMBER} \
                      -t ${IMAGE_NAME}:latest \
                      .
                """
            }
        }
        stage('Push') {
            steps {
                sh """
                    docker push ${IMAGE_NAME}:${BUILD_NUMBER}
                    docker push ${IMAGE_NAME}:latest
                """
            }
        }
    }
}