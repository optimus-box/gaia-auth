use std::fs;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json, RequestPartsExt,
};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{controller::Errors, model::UserWithGroups};
static PERMISSIONS: &[&str] = &["root", "admin"];

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub groups: Vec<String>,
}

pub struct Jwt {
    pub id: uuid::Uuid,
    pub perms: Vec<String>,
}

pub fn generate_token(user: &UserWithGroups) -> Result<String, (StatusCode, Json<Errors>)> {
    let now = Utc::now().timestamp();
    let exp = now + 60 * 60 * 24; // 1 day
    let header = Header::new(Algorithm::RS256);
    let private_key_file = std::env::var("JWT_PRIVATE_KEY").unwrap_or(String::from("private.pem"));
    let mut permissions = vec![];

    for group in &user.groups {
        permissions.append(&mut group.permissions());
    }

    let claims = Claims {
        iss: std::env::var("JWT_ISSUER").unwrap_or(String::from("gaia")),
        sub: user.user.id.to_string(),
        exp,
        iat: now,
        groups: permissions,
    };

    let data = match fs::read(private_key_file) {
        Ok(data) => data,
        Err(err) => return Err(Errors::internal(&err.to_string())),
    };

    let private_key = match EncodingKey::from_rsa_pem(&data) {
        Ok(key) => key,
        Err(err) => return Err(Errors::internal(&err.to_string())),
    };

    match jsonwebtoken::encode(&header, &claims, &private_key) {
        Ok(token) => Ok(token),
        Err(err) => Err(Errors::internal(&err.to_string())),
    }
}

pub fn verify_token(token: &str) -> Result<Claims, (StatusCode, Json<Errors>)> {
    let public_key_file = std::env::var("JWT_PUBLIC_KEY").unwrap_or(String::from("public.pem"));
    let data = match fs::read(public_key_file) {
        Ok(data) => data,
        Err(err) => return Err(Errors::internal(&err.to_string())),
    };
    let key = DecodingKey::from_rsa_pem(&data).map_err(|err| Errors::internal(&err.to_string()))?;

    match jsonwebtoken::decode(token, &key, &Validation::new(Algorithm::RS256)) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(Errors::internal(&err.to_string())),
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Jwt
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Errors>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|err| Errors::unauthorized(&err.to_string()))?;
        let claims = verify_token(bearer.token())?;
        let id: uuid::Uuid =
            uuid::Uuid::parse_str(&claims.sub).map_err(|err| Errors::internal(&err.to_string()))?;
        Ok(Jwt {
            id,
            perms: claims.groups,
        })
    }
}

impl Jwt {
    fn has_admin(&self) -> bool {
        self.perms.iter().any(|p| PERMISSIONS.contains(&p.as_str()))
    }

    pub fn is_root(&self) -> bool {
        self.perms.contains(&String::from("root"))
    }

    pub fn is_admin(&self) -> bool {
        self.perms.contains(&String::from("admin")) || self.is_root()
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        if !self.perms.contains(&String::from(permission)) {
            return self.has_admin();
        }
        true
    }

    pub fn has_permissions(&self, permissions: &[&str]) -> bool {
        if !self.perms.iter().any(|p| permissions.contains(&p.as_str())) {
            return self.has_admin();
        }
        true
    }
}
