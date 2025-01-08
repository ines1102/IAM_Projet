const Keycloak = require('keycloak-connect');
const session = require('express-session');
const FileStore = require('session-file-store')(session);

module.exports = function (app) {
  const keycloak = new Keycloak({}, {
    clientId: 'my_app',
    bearerOnly: true,
    serverUrl: 'http://localhost:8080',
    realm: 'my_realm',
    credentials: {
      secret: 'your-client-secret'
    }
  });

  // Utiliser une session pour stocker le token Keycloak
  app.use(session({
    secret: 'your-session-secret',
    resave: false,
    saveUninitialized: true,
    store: new FileStore()
  }));

  // Initialiser Keycloak
  app.use(keycloak.middleware({
    logout: '/logout',
    admin: '/admin'
  }));

  return keycloak;
};