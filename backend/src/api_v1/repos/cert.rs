use std::sync::Arc;
use uuid::Uuid;
use anyhow::{Result, Error};
use sea_orm::{
    ActiveValue::Set, 
    DatabaseConnection, 
    EntityTrait,
    PaginatorTrait, 
    QuerySelect, 
    SqlErr
};
use crate::{
    api_v1::models::cert, 
    utils::log_error::ResultLogger
};

pub struct CertRepo {
    database: Arc<DatabaseConnection>
}

pub struct CertModel {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub title: String
}

pub enum CreationError {
    UniqueErr,
    Another(Error)
}

impl CertRepo {
    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self {
            database
        }
    }

    pub async fn create_cert(&self, cert: CertModel) -> Result<Uuid, CreationError> {
        let model_to_insert = cert::ActiveModel {
            id: Set(cert.id),
            email: Set(cert.email),
            name: Set(cert.name),
            title: Set(cert.title)
        };

        let created_cert_or_error = cert::Entity::insert(model_to_insert)
            .exec(self.database.as_ref())
            .await
            .log_with_place_on_error("create_cert");

        match created_cert_or_error {
            Ok(cert) => Ok(cert.last_insert_id),
            Err(err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                    Err(CreationError::UniqueErr)
                } else {
                    Err(CreationError::Another(err.into()))
                }
            }
        }
    }

    pub async fn find_cert_by_id(&self, id: Uuid) -> Result<Option<CertModel>> {
        let search_result = cert::Entity::find_by_id(id)
            .limit(1)
            .one(self.database.as_ref())
            .await
            .log_with_place_on_error("find_cert_by_id")?;

        if let Some(cert) = search_result {
            Ok(Some(CertModel { 
                id, 
                email: cert.email, 
                name: cert.name, 
                title: cert.title
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_cert_by_email(&self, email: String) -> Result<Option<CertModel>> {
        let search_result = cert::Entity::find_by_email(email.to_string())
            .limit(1)
            .one(self.database.as_ref())
            .await
            .log_with_place_on_error("find_cert_by_email")?;

        if let Some(cert) = search_result {
            Ok(Some(CertModel { 
                id: cert.id, 
                email, 
                name: cert.name, 
                title: cert.title
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn remove_cert_by_id(&self, id: Uuid) -> Result<u64> {
        Ok(
            cert::Entity::delete_by_id(id)
                .exec(self.database.as_ref())
                .await
                .log_with_place_on_error("remove_cert_by_id")?
                .rows_affected
        )
    }

    pub async fn remove_cert_by_email(&self, email: String) -> Result<u64> {
        Ok(
            cert::Entity::delete(cert::ActiveModel {
                email: Set(email),
                ..Default::default()
            })
                .exec(self.database.as_ref())
                .await
                .log_with_place_on_error("remove_cert_by_email")?
                .rows_affected
        )
    }

    pub async fn remove_cert_by_id_and_email(&self, id: Uuid, email: String) -> Result<u64> {
        Ok(
            cert::Entity::delete(cert::ActiveModel {
                id: Set(id),
                email: Set(email),
                ..Default::default()
            })
                .exec(self.database.as_ref())
                .await
                .log_with_place_on_error("remove_cert_by_id_and_email")?
                .rows_affected
        )
    }

    pub async fn count_all(&self) -> Result<u64> {
        let count: u64 = cert::Entity::find()
            .count(self.database.as_ref())
            .await?;

        Ok(count)
    }
 }
