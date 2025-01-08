const express = require('express');
const session = require('express-session');
const { keycloak, memoryStore } = require('./keycloak-config');
const app = express();

// Configuration de la session
app.use(session({
  secret: 'some-secret',
  resave: false,
  saveUninitialized: true,
  store: memoryStore,
}));

// Configuration de Keycloak avec le middleware
app.use(keycloak.middleware({
  logout: '/logout',
  admin: '/',
}));

// Route de test, redirection vers Keycloak pour l'authentification
app.get('/login', keycloak.protect(), (req, res) => {
  res.send('Vous êtes connecté !');
});

// Route protégée, accessible uniquement après authentification
app.get('/protected', keycloak.protect(), (req, res) => {
  res.send('Vous avez accès à cette route protégée !');
});

// Route pour tester si l'utilisateur est connecté et afficher le jeton
app.get('/token', (req, res) => {
  const token = req.session['keycloak-token'];
  if (token) {
    res.json({ token: token });
  } else {
    res.status(401).send('Pas de jeton, utilisateur non authentifié.');
  }
});

// Démarrer le serveur
app.listen(3000, () => {
  console.log('Application en écoute sur le port 3000');
});