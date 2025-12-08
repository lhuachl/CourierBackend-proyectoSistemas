use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreatePerfilClienteDTO, UpdatePerfilClienteDTO, PerfilClienteResponseDTO,
    PerfilesClienteListResponseDTO,
};
use crate::domain::repositories::PerfilClienteRepository;
use crate::shared::error::{AppError, AppResult};

/// Service que maneja la lógica de negocio de perfiles de cliente
pub struct PerfilClienteService {
    repository: Arc<dyn PerfilClienteRepository>,
}

impl PerfilClienteService {
    pub fn new(repository: Arc<dyn PerfilClienteRepository>) -> Self {
        Self { repository }
    }

    /// Obtiene el perfil del usuario actual
    pub async fn get_my_perfil(&self, user_id: Uuid) -> AppResult<PerfilClienteResponseDTO> {
        let perfil = self
            .repository
            .find_by_usuario(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Perfil no encontrado. Debe crear uno primero.".into()))?;

        Ok(perfil.into())
    }

    /// Obtiene un perfil por su ID (admin)
    pub async fn get_perfil_by_id(&self, id: Uuid) -> AppResult<PerfilClienteResponseDTO> {
        let perfil = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Perfil con ID {} no encontrado", id)))?;

        Ok(perfil.into())
    }

    /// Lista todos los perfiles (admin)
    pub async fn list_perfiles(&self) -> AppResult<PerfilesClienteListResponseDTO> {
        let perfiles = self.repository.find_all().await?;
        Ok(perfiles.into())
    }

    /// Crea un nuevo perfil para el usuario actual
    pub async fn create_perfil(
        &self,
        user_id: Uuid,
        dto: CreatePerfilClienteDTO,
    ) -> AppResult<PerfilClienteResponseDTO> {
        // Verificar si ya existe un perfil para este usuario
        if self.repository.exists_for_usuario(user_id).await? {
            return Err(AppError::BadRequest(
                "Ya existe un perfil para este usuario".into(),
            ));
        }

        // Validar documento de identidad único si se proporciona
        if let Some(ref doc) = dto.documento_identidad {
            if !doc.is_empty() {
                if let Some(_existing) = self.repository.find_by_documento(doc).await? {
                    return Err(AppError::BadRequest(
                        "El documento de identidad ya está registrado".into(),
                    ));
                }
                
                // Validar formato de documento (básico)
                if doc.len() < 5 || doc.len() > 20 {
                    return Err(AppError::BadRequest(
                        "El documento de identidad debe tener entre 5 y 20 caracteres".into(),
                    ));
                }
            }
        }

        // Validar teléfono si se proporciona
        if let Some(ref tel) = dto.telefono {
            if !tel.is_empty() && !Self::validar_telefono(tel) {
                return Err(AppError::BadRequest(
                    "Formato de teléfono inválido. Use formato internacional (+593...)".into(),
                ));
            }
        }

        let perfil = self
            .repository
            .create(
                user_id,
                dto.documento_identidad.as_deref(),
                dto.telefono.as_deref(),
            )
            .await?;

        tracing::info!("Perfil creado para usuario {}", user_id);
        Ok(perfil.into())
    }

    /// Actualiza el perfil del usuario actual
    pub async fn update_my_perfil(
        &self,
        user_id: Uuid,
        dto: UpdatePerfilClienteDTO,
    ) -> AppResult<PerfilClienteResponseDTO> {
        // Buscar el perfil del usuario
        let perfil = self
            .repository
            .find_by_usuario(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Perfil no encontrado".into()))?;

        // Validar documento si se actualiza
        if let Some(ref doc) = dto.documento_identidad {
            if !doc.is_empty() {
                // Verificar que no esté en uso por otro usuario
                if let Some(existing) = self.repository.find_by_documento(doc).await? {
                    if existing.id_perfil != perfil.id_perfil {
                        return Err(AppError::BadRequest(
                            "El documento de identidad ya está registrado".into(),
                        ));
                    }
                }
                
                if doc.len() < 5 || doc.len() > 20 {
                    return Err(AppError::BadRequest(
                        "El documento de identidad debe tener entre 5 y 20 caracteres".into(),
                    ));
                }
            }
        }

        // Validar teléfono si se actualiza
        if let Some(ref tel) = dto.telefono {
            if !tel.is_empty() && !Self::validar_telefono(tel) {
                return Err(AppError::BadRequest(
                    "Formato de teléfono inválido. Use formato internacional (+593...)".into(),
                ));
            }
        }

        let updated = self
            .repository
            .update(
                perfil.id_perfil,
                dto.documento_identidad.as_deref(),
                dto.telefono.as_deref(),
            )
            .await?;

        tracing::info!("Perfil {} actualizado", perfil.id_perfil);
        Ok(updated.into())
    }

    /// Actualiza un perfil por ID (admin)
    pub async fn update_perfil_by_id(
        &self,
        id: Uuid,
        dto: UpdatePerfilClienteDTO,
    ) -> AppResult<PerfilClienteResponseDTO> {
        // Verificar que existe
        let perfil = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Perfil con ID {} no encontrado", id)))?;

        // Validar documento si se actualiza
        if let Some(ref doc) = dto.documento_identidad {
            if !doc.is_empty() {
                if let Some(existing) = self.repository.find_by_documento(doc).await? {
                    if existing.id_perfil != perfil.id_perfil {
                        return Err(AppError::BadRequest(
                            "El documento de identidad ya está registrado".into(),
                        ));
                    }
                }
            }
        }

        let updated = self
            .repository
            .update(
                id,
                dto.documento_identidad.as_deref(),
                dto.telefono.as_deref(),
            )
            .await?;

        Ok(updated.into())
    }

    /// Elimina el perfil del usuario actual
    pub async fn delete_my_perfil(&self, user_id: Uuid) -> AppResult<()> {
        let perfil = self
            .repository
            .find_by_usuario(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Perfil no encontrado".into()))?;

        self.repository.delete(perfil.id_perfil).await?;
        tracing::info!("Perfil {} eliminado", perfil.id_perfil);
        Ok(())
    }

    /// Elimina un perfil por ID (admin)
    pub async fn delete_perfil_by_id(&self, id: Uuid) -> AppResult<()> {
        // Verificar que existe
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Perfil con ID {} no encontrado", id)))?;

        self.repository.delete(id).await?;
        Ok(())
    }

    // ========================================================================
    // HELPERS PRIVADOS
    // ========================================================================

    /// Valida el formato de un número de teléfono
    fn validar_telefono(telefono: &str) -> bool {
        // Formato básico: debe empezar con + y tener entre 10 y 15 dígitos
        let cleaned: String = telefono.chars().filter(|c| c.is_ascii_digit()).collect();
        
        if telefono.starts_with('+') {
            cleaned.len() >= 10 && cleaned.len() <= 15
        } else {
            // También aceptar formato local (10 dígitos)
            cleaned.len() >= 9 && cleaned.len() <= 15
        }
    }
}
