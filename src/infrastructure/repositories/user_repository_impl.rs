use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at 
             FROM public.users 
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at 
             FROM public.users 
             WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at 
             FROM public.users 
             WHERE activo = true
             ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_all_paginated(&self, limit: i64, offset: i64) -> Result<(Vec<User>, i64), sqlx::Error> {
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM public.users WHERE activo = true"
        )
        .fetch_one(&self.pool)
        .await?;

        let users = sqlx::query_as::<_, User>(
            "SELECT id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at 
             FROM public.users 
             WHERE activo = true
             ORDER BY created_at DESC
             LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((users, total))
    }

    async fn create(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO public.users (id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at"
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.nombre)
        .bind(&user.apellido)
        .bind(&user.rol)
        .bind(&user.foto_perfil)
        .bind(user.activo)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE public.users 
             SET email = $1, nombre = $2, apellido = $3, rol = $4, foto_perfil = $5, updated_at = $6
             WHERE id = $7
             RETURNING id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at"
        )
        .bind(&user.email)
        .bind(&user.nombre)
        .bind(&user.apellido)
        .bind(&user.rol)
        .bind(&user.foto_perfil)
        .bind(chrono::Utc::now())
        .bind(user.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_role(&self, id: Uuid, rol: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE public.users 
             SET rol = $1, updated_at = $2
             WHERE id = $3
             RETURNING id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at"
        )
        .bind(rol)
        .bind(chrono::Utc::now())
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_status(&self, id: Uuid, activo: bool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE public.users 
             SET activo = $1, updated_at = $2
             WHERE id = $3
             RETURNING id, email, nombre, apellido, rol, foto_perfil, activo, created_at, updated_at"
        )
        .bind(activo)
        .bind(chrono::Utc::now())
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE public.users 
             SET activo = false, updated_at = $1
             WHERE id = $2"
        )
        .bind(chrono::Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
