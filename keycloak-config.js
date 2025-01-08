const Keycloak = require('keycloak-connect');
const session = require('express-session');

// Déclare d'abord memoryStore avant de l'utiliser
const memoryStore = new session.MemoryStore();

// Initialiser Keycloak avec la session
const keycloak = new Keycloak({
  store: memoryStore, // Nous utilisons la session en mémoire pour cet exemple
});

module.exports = {
  keycloak,
  memoryStore
};