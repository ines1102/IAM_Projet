CREATE DATABASE keycloak;

CREATE USER keycloak_user
WITH
    PASSWORD 'your_password';

GRANT ALL PRIVILEGES ON DATABASE keycloak TO keycloak_user;