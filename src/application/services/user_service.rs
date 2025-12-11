use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreateUserDTO, UpdateUserDTO, UpdateUserRoleDTO, UpdateUserStatusDTO,
    UserResponseDTO, UsersListResponseDTO,
};
use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::shared::error::{AppError, AppResult};

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    /// Obtiene un usuario por ID
    pub async fn get_user(&self, id: Uuid) -> AppResult<UserResponseDTO> {
        let user = self.repository
            .find_by_id(id)
            .await
            .map_err(|_| AppError::NotFound("Usuario no encontrado".to_string()))?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        Ok(UserResponseDTO::from(user))
    }

    /// Obtiene un usuario por email
    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<UserResponseDTO>> {
        let user = self.repository
            .find_by_email(email)
            .await
            .map_err(|_| AppError::Internal("Error al buscar usuario".to_string()))?;

        Ok(user.map(UserResponseDTO::from))
    }

    /// Lista todos los usuarios
    pub async fn list_users(&self) -> AppResult<UsersListResponseDTO> {
        let users = self.repository
            .find_all()
            .await
            .map_err(|_| AppError::Internal("Error al listar usuarios".to_string()))?;

        let total = users.len() as i64;
        let user_dtos = users.into_iter().map(UserResponseDTO::from).collect();

        Ok(UsersListResponseDTO {
            total,
            users: user_dtos,
        })
    }

    /// Lista usuarios con paginación
    pub async fn list_users_paginated(&self, limit: i64, offset: i64) -> AppResult<UsersListResponseDTO> {
        let (users, total) = self.repository
            .find_all_paginated(limit, offset)
            .await
            .map_err(|_| AppError::Internal("Error al listar usuarios".to_string()))?;

        let user_dtos = users.into_iter().map(UserResponseDTO::from).collect();

        Ok(UsersListResponseDTO {
            total,
            users: user_dtos,
        })
    }

    /// Crea un nuevo usuario (desde Supabase webhook)
    pub async fn create_user(&self, dto: CreateUserDTO) -> AppResult<UserResponseDTO> {
        // Verificar que el usuario no existe
        if let Ok(Some(_)) = self.repository.find_by_email(&dto.email).await {
            return Err(AppError::BadRequest("Usuario ya existe".to_string()));
        }

        let user = User {
            id: Uuid::new_v4(),
            email: Some(dto.email),
            nombre: dto.nombre,
            apellido: dto.apellido,
            rol: dto.rol,
            foto_perfil: None,
            activo: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let created = self.repository
            .create(&user)
            .await
            .map_err(|_| AppError::Internal("Error al crear usuario".to_string()))?;

        Ok(UserResponseDTO::from(created))
    }

    /// Actualiza un usuario
    pub async fn update_user(&self, id: Uuid, dto: UpdateUserDTO) -> AppResult<UserResponseDTO> {
        let mut user = self.repository
            .find_by_id(id)
            .await
            .map_err(|_| AppError::Internal("Error al obtener usuario".to_string()))?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        if let Some(email) = dto.email {
            user.email = Some(email);
        }
        if let Some(nombre) = dto.nombre {
            user.nombre = Some(nombre);
        }
        if let Some(apellido) = dto.apellido {
            user.apellido = Some(apellido);
        }
        if let Some(foto_perfil) = dto.foto_perfil {
            user.foto_perfil = Some(foto_perfil);
        }
        if let Some(rol) = dto.rol {
            user.rol = rol;
        }

        let updated = self.repository
            .update(&user)
            .await
            .map_err(|_| AppError::Internal("Error al actualizar usuario".to_string()))?;

        Ok(UserResponseDTO::from(updated))
    }

    /// Actualiza el rol de un usuario (solo admin)
    pub async fn update_user_role(&self, id: Uuid, dto: UpdateUserRoleDTO) -> AppResult<UserResponseDTO> {
        let updated = self.repository
            .update_role(id, &dto.rol)
            .await
            .map_err(|_| AppError::Internal("Error al actualizar rol".to_string()))?;

        Ok(UserResponseDTO::from(updated))
    }

    /// Actualiza el estado de un usuario (solo admin)
    pub async fn update_user_status(&self, id: Uuid, dto: UpdateUserStatusDTO) -> AppResult<UserResponseDTO> {
        let updated = self.repository
            .update_status(id, dto.activo)
            .await
            .map_err(|_| AppError::Internal("Error al actualizar estado".to_string()))?;

        Ok(UserResponseDTO::from(updated))
    }

    /// Elimina un usuario (soft delete)
    pub async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        self.repository
            .delete(id)
            .await
            .map_err(|_| AppError::Internal("Error al eliminar usuario".to_string()))?;

        Ok(())
    }
}

// ============================================================================
// CONVERSIONES
// ============================================================================

impl From<User> for UserResponseDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            nombre: user.nombre,
            apellido: user.apellido,
            rol: user.rol,
            foto_perfil: user.foto_perfil,
            activo: user.activo,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
