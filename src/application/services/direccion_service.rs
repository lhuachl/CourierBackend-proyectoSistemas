use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreateDireccionDTO, UpdateDireccionDTO, CreateAlmacenDTO,
    DireccionResponseDTO, DireccionesListResponseDTO,
};
use crate::domain::repositories::DireccionRepository;
use crate::shared::error::{AppError, AppResult};

/// Número máximo de direcciones por perfil
const MAX_DIRECCIONES_POR_PERFIL: i64 = 10;

/// Service que maneja la lógica de negocio de direcciones
pub struct DireccionService {
    repository: Arc<dyn DireccionRepository>,
}

impl DireccionService {
    pub fn new(repository: Arc<dyn DireccionRepository>) -> Self {
        Self { repository }
    }

    // ========================================================================
    // CONSULTAS - CLIENTE
    // ========================================================================

    /// Lista las direcciones del usuario actual
    pub async fn list_my_direcciones(&self, id_perfil: Uuid) -> AppResult<DireccionesListResponseDTO> {
        let direcciones = self.repository.find_by_perfil(id_perfil).await?;
        Ok(direcciones.into())
    }

    /// Lista todas las direcciones del usuario (incluye inactivas)
    pub async fn list_all_my_direcciones(&self, id_perfil: Uuid) -> AppResult<DireccionesListResponseDTO> {
        let direcciones = self.repository.find_all_by_perfil(id_perfil).await?;
        Ok(direcciones.into())
    }

    /// Obtiene una dirección por ID (verificando propiedad)
    pub async fn get_direccion(&self, id: Uuid, id_perfil: Uuid) -> AppResult<DireccionResponseDTO> {
        let direccion = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Dirección {} no encontrada", id)))?;

        // Verificar propiedad (las direcciones de almacén no tienen id_perfil)
        if direccion.tipo == "cliente" {
            if direccion.id_perfil != Some(id_perfil) {
                return Err(AppError::Forbidden("No tiene acceso a esta dirección".into()));
            }
        }

        Ok(direccion.into())
    }

    /// Obtiene la dirección predeterminada del usuario
    pub async fn get_predeterminada(&self, id_perfil: Uuid) -> AppResult<DireccionResponseDTO> {
        let direccion = self
            .repository
            .find_predeterminada(id_perfil)
            .await?
            .ok_or_else(|| AppError::NotFound("No tiene dirección predeterminada".into()))?;

        Ok(direccion.into())
    }

    // ========================================================================
    // CONSULTAS - ALMACENES (público)
    // ========================================================================

    /// Lista los almacenes disponibles (para selección de origen)
    pub async fn list_almacenes(&self) -> AppResult<DireccionesListResponseDTO> {
        let almacenes = self.repository.find_almacenes().await?;
        Ok(almacenes.into())
    }

    // ========================================================================
    // MUTACIONES - CLIENTE
    // ========================================================================

    /// Crea una nueva dirección para el usuario
    pub async fn create_direccion(
        &self,
        id_perfil: Uuid,
        dto: CreateDireccionDTO,
    ) -> AppResult<DireccionResponseDTO> {
        // Validar límite de direcciones
        let count = self.repository.count_by_perfil(id_perfil).await?;
        if count >= MAX_DIRECCIONES_POR_PERFIL {
            return Err(AppError::BadRequest(format!(
                "Límite de {} direcciones alcanzado", MAX_DIRECCIONES_POR_PERFIL
            )));
        }

        // Validar campos requeridos
        if dto.calle.trim().is_empty() {
            return Err(AppError::BadRequest("La calle es requerida".into()));
        }
        if dto.ciudad.trim().is_empty() {
            return Err(AppError::BadRequest("La ciudad es requerida".into()));
        }

        // Validar coordenadas
        Self::validar_coordenadas(dto.latitud, dto.longitud)?;

        // Determinar tipo (default: cliente)
        let tipo = dto.tipo.as_deref().unwrap_or("cliente");
        if tipo != "cliente" {
            return Err(AppError::BadRequest("Solo puede crear direcciones de tipo cliente".into()));
        }

        // Si es predeterminada y hay otras, quitar el flag de las demás
        let es_predeterminada = dto.es_predeterminada.unwrap_or(false);
        if es_predeterminada {
            if let Some(actual) = self.repository.find_predeterminada(id_perfil).await? {
                self.repository.set_predeterminada(actual.id_direccion, id_perfil).await?;
            }
        }

        // Convertir coordenadas a Decimal
        let latitud = Decimal::try_from(dto.latitud)
            .map_err(|_| AppError::BadRequest("Latitud inválida".into()))?;
        let longitud = Decimal::try_from(dto.longitud)
            .map_err(|_| AppError::BadRequest("Longitud inválida".into()))?;

        let direccion = self
            .repository
            .create(
                Some(id_perfil),
                tipo,
                dto.calle.trim(),
                dto.ciudad.trim(),
                dto.pais.as_deref().unwrap_or("Ecuador"),
                dto.referencias_adicionales.as_deref(),
                latitud,
                longitud,
                es_predeterminada || count == 0, // Primera dirección siempre es predeterminada
            )
            .await?;

        tracing::info!("Dirección creada para perfil {}: {}", id_perfil, direccion.id_direccion);
        Ok(direccion.into())
    }

    /// Actualiza una dirección del usuario
    pub async fn update_direccion(
        &self,
        id: Uuid,
        id_perfil: Uuid,
        dto: UpdateDireccionDTO,
    ) -> AppResult<DireccionResponseDTO> {
        // Verificar propiedad
        if !self.repository.belongs_to_perfil(id, id_perfil).await? {
            return Err(AppError::Forbidden("No tiene acceso a esta dirección".into()));
        }

        // Validar coordenadas si se actualizan
        if let (Some(lat), Some(lng)) = (dto.latitud, dto.longitud) {
            Self::validar_coordenadas(lat, lng)?;
        }

        let latitud = dto.latitud.map(|l| Decimal::try_from(l).ok()).flatten();
        let longitud = dto.longitud.map(|l| Decimal::try_from(l).ok()).flatten();

        let direccion = self
            .repository
            .update(
                id,
                dto.calle.as_deref(),
                dto.ciudad.as_deref(),
                dto.pais.as_deref(),
                dto.referencias_adicionales.as_deref(),
                latitud,
                longitud,
            )
            .await?;

        tracing::info!("Dirección {} actualizada", id);
        Ok(direccion.into())
    }

    /// Establece una dirección como predeterminada
    pub async fn set_predeterminada(&self, id: Uuid, id_perfil: Uuid) -> AppResult<DireccionResponseDTO> {
        // Verificar propiedad
        if !self.repository.belongs_to_perfil(id, id_perfil).await? {
            return Err(AppError::Forbidden("No tiene acceso a esta dirección".into()));
        }

        // Verificar que está activa
        let direccion = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Dirección no encontrada".into()))?;

        if !direccion.activo {
            return Err(AppError::BadRequest("No puede establecer como predeterminada una dirección inactiva".into()));
        }

        let updated = self.repository.set_predeterminada(id, id_perfil).await?;
        tracing::info!("Dirección {} establecida como predeterminada para perfil {}", id, id_perfil);
        Ok(updated.into())
    }

    /// Desactiva una dirección (soft delete)
    pub async fn deactivate_direccion(&self, id: Uuid, id_perfil: Uuid) -> AppResult<()> {
        // Verificar propiedad
        if !self.repository.belongs_to_perfil(id, id_perfil).await? {
            return Err(AppError::Forbidden("No tiene acceso a esta dirección".into()));
        }

        self.repository.deactivate(id).await?;
        tracing::info!("Dirección {} desactivada", id);
        Ok(())
    }

    /// Reactiva una dirección
    pub async fn activate_direccion(&self, id: Uuid, id_perfil: Uuid) -> AppResult<DireccionResponseDTO> {
        // Verificar propiedad
        if !self.repository.belongs_to_perfil(id, id_perfil).await? {
            return Err(AppError::Forbidden("No tiene acceso a esta dirección".into()));
        }

        let direccion = self.repository.activate(id).await?;
        tracing::info!("Dirección {} reactivada", id);
        Ok(direccion.into())
    }

    /// Elimina una dirección permanentemente (hard delete)
    pub async fn delete_direccion(&self, id: Uuid, id_perfil: Uuid) -> AppResult<()> {
        // Verificar propiedad
        if !self.repository.belongs_to_perfil(id, id_perfil).await? {
            return Err(AppError::Forbidden("No tiene acceso a esta dirección".into()));
        }

        // TODO: Verificar que no esté asociada a pedidos activos

        self.repository.delete(id).await?;
        tracing::info!("Dirección {} eliminada permanentemente", id);
        Ok(())
    }

    // ========================================================================
    // MUTACIONES - ADMIN (almacenes)
    // ========================================================================

    /// Lista todos los almacenes (incluye inactivos) - Admin
    pub async fn list_all_almacenes(&self) -> AppResult<DireccionesListResponseDTO> {
        let almacenes = self.repository.find_all_almacenes().await?;
        Ok(almacenes.into())
    }

    /// Crea un nuevo almacén - Admin
    pub async fn create_almacen(&self, dto: CreateAlmacenDTO) -> AppResult<DireccionResponseDTO> {
        // Validar campos requeridos
        if dto.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre del almacén es requerido".into()));
        }
        if dto.calle.trim().is_empty() {
            return Err(AppError::BadRequest("La calle es requerida".into()));
        }
        if dto.ciudad.trim().is_empty() {
            return Err(AppError::BadRequest("La ciudad es requerida".into()));
        }

        Self::validar_coordenadas(dto.latitud, dto.longitud)?;

        let latitud = Decimal::try_from(dto.latitud)
            .map_err(|_| AppError::BadRequest("Latitud inválida".into()))?;
        let longitud = Decimal::try_from(dto.longitud)
            .map_err(|_| AppError::BadRequest("Longitud inválida".into()))?;

        // Combinar nombre con calle para identificación
        let calle_completa = format!("{} - {}", dto.nombre.trim(), dto.calle.trim());

        let almacen = self
            .repository
            .create(
                None, // Los almacenes no tienen perfil
                "almacen",
                &calle_completa,
                dto.ciudad.trim(),
                dto.pais.as_deref().unwrap_or("Ecuador"),
                dto.referencias_adicionales.as_deref(),
                latitud,
                longitud,
                false, // Los almacenes no son "predeterminados"
            )
            .await?;

        tracing::info!("Almacén creado: {} ({})", dto.nombre, almacen.id_direccion);
        Ok(almacen.into())
    }

    /// Desactiva un almacén - Admin
    pub async fn deactivate_almacen(&self, id: Uuid) -> AppResult<()> {
        let direccion = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Almacén no encontrado".into()))?;

        if direccion.tipo != "almacen" {
            return Err(AppError::BadRequest("La dirección no es un almacén".into()));
        }

        self.repository.deactivate(id).await?;
        tracing::info!("Almacén {} desactivado", id);
        Ok(())
    }

    /// Reactiva un almacén - Admin
    pub async fn activate_almacen(&self, id: Uuid) -> AppResult<DireccionResponseDTO> {
        let direccion = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Almacén no encontrado".into()))?;

        if direccion.tipo != "almacen" {
            return Err(AppError::BadRequest("La dirección no es un almacén".into()));
        }

        let almacen = self.repository.activate(id).await?;
        tracing::info!("Almacén {} reactivado", id);
        Ok(almacen.into())
    }

    /// Elimina un almacén permanentemente - Admin
    pub async fn delete_almacen(&self, id: Uuid) -> AppResult<()> {
        let direccion = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Almacén no encontrado".into()))?;

        if direccion.tipo != "almacen" {
            return Err(AppError::BadRequest("La dirección no es un almacén".into()));
        }

        // TODO: Verificar que no esté asociado a pedidos activos

        self.repository.delete(id).await?;
        tracing::info!("Almacén {} eliminado permanentemente", id);
        Ok(())
    }

    // ========================================================================
    // HELPERS PRIVADOS
    // ========================================================================

    /// Valida coordenadas geográficas
    fn validar_coordenadas(latitud: f64, longitud: f64) -> AppResult<()> {
        // Latitud: -90 a 90
        if !(-90.0..=90.0).contains(&latitud) {
            return Err(AppError::BadRequest("Latitud debe estar entre -90 y 90".into()));
        }
        // Longitud: -180 a 180
        if !(-180.0..=180.0).contains(&longitud) {
            return Err(AppError::BadRequest("Longitud debe estar entre -180 y 180".into()));
        }
        Ok(())
    }
}
