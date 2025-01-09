import jwt
import requests
from jwt.algorithms import RSAAlgorithm

# URL pour récupérer les clés publiques de Keycloak
certs_url = "http://localhost:8080/realms/myrealm/protocol/openid-connect/certs"

# Fonction pour récupérer les clés publiques de Keycloak
def get_public_key(kid):
    response = requests.get(certs_url)
    if response.status_code == 200:
        keys = response.json().get('keys', [])
        for key in keys:
            if key['kid'] == kid:
                return key
    return None

# Nouveau Token JWT à vérifier (assurez-vous qu'il est correctement formaté)
token = "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJiMnpqa0ZGTFNBNDVrSkV1dF8yb05VMFF2dnRVd1paQjBOQmpGZ2E3dXhRIn0.eyJleHAiOjE3MzY0NTYxMDMsImlhdCI6MTczNjQ1NTgwMywianRpIjoiZGFkZjE1OGYtYzQ2Ni00MzZiLWE4ZjUtNmU5ODIxNzRhZjM4IiwiaXNzIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwL3JlYWxtcy9teXJlYWxtIiwiYXVkIjoiYWNjb3VudCIsInN1YiI6IjE4YjJjZTQ2LTFmNGYtNGI0NS04NjNmLTM0ODVjOGQ0YmViNyIsInR5cCI6IkJlYXJlciIsImF6cCI6Im15Y2xpZW50Iiwic2lkIjoiMzQ5MmY3ZTctNDk3YS00OWYyLWFiY2QtNzZkNGZlMGE0YjZmIiwiYWNyIjoiMSIsImFsbG93ZWQtb3JpZ2lucyI6WyJodHRwOi8vbG9jYWxob3N0OjgwMDAiLCJodHRwczovL3d3dy5rZXljbG9hay5vcmciXSwicmVhbG1fYWNjZXNzIjp7InJvbGVzIjpbImRlZmF1bHQtcm9sZXMtbXlyZWFsbSIsIm9mZmxpbmVfYWNjZXNzIiwidW1hX2F1dGhvcml6YXRpb24iXX0sInJlc291cmNlX2FjY2VzcyI6eyJhY2NvdW50Ijp7InJvbGVzIjpbIm1hbmFnZS1hY2NvdW50IiwibWFuYWdlLWFjY291bnQtbGlua3MiLCJ2aWV3LXByb2ZpbGUiXX19LCJzY29wZSI6Im9wZW5pZCBwcm9maWxlIGVtYWlsIiwiZW1haWxfdmVyaWZpZWQiOmZhbHNlLCJuYW1lIjoiRmlyc3QgTGFzdCIsInByZWZlcnJlZF91c2VybmFtZSI6ImV0dWRpYW50IiwiZ2l2ZW5fbmFtZSI6IkZpcnN0IiwiZmFtaWx5X25hbWUiOiJMYXN0IiwiZW1haWwiOiJldHVkaWFudEBpdHMuY29tIn0.srPp3yPPNaOGxqX47XWJi5-kQbN1JYoWo74BUe7kfqFX9hfi4hNbMXY6BKja7W5_9KmJZueqCVQckfb08qtO7vzDj2jH0uQ1DR2cV3n--76uux82YXRPVSBF-hRh2HG8N7g-rpzZWuX3rOPEYazmDOMSNV8XHX5D4GSfSMA68o2ZN2SI6DYHjqoBis-5Vdb4pQX8w7cTaK78kmbwyUxRIkml9p6Ehf4swrTOhQSHf4iCd3dOq74PnVexnIkga-8nufi5DdgjsYOm2JgqQxjRanJpdKmrEV61i-TC04gZJ_Ir9XKFUEZ5vV0awHMcDrYURQnosoKKZrjnwPLVuYGGnQ"

# Décodez le token JWT sans vérifier la signature pour obtenir le kid
decoded_token_without_verification = jwt.decode(token, options={"verify_signature": False})
print("Decoded token without verification:", decoded_token_without_verification)

# Récupérez le kid du token JWT
kid = jwt.get_unverified_header(token).get('kid')
print("Key ID (kid):", kid)

# Récupérez la clé publique correspondant au kid
key = get_public_key(kid)

if key:
    try:
        # Charger la clé publique
        rsa_key = RSAAlgorithm.from_jwk(key)

        # Vérifier le token JWT
        decoded_token = jwt.decode(token, rsa_key, algorithms=["RS256"], issuer="http://localhost:8080/realms/myrealm", audience="account")
        if decoded_token:
            print("Token is valid:", decoded_token)
    except jwt.ExpiredSignatureError:
        print("Token has expired")
    except jwt.InvalidAudienceError:
        print("Invalid audience")
    except jwt.InvalidIssuerError:
        print("Invalid issuer")
    except jwt.DecodeError as e:
        print(f"Error decoding token: {e}")
    except jwt.InvalidTokenError as e:
        print(f"Invalid token: {e}")
else:
    print("Public key not found for kid:", kid)