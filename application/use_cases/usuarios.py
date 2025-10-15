from dataclasses import dataclass
from typing import List, Optional
from uuid import UUID

from domain.repositories.user_repository import IUserRepository
from models.usuarios import User


@dataclass
class CrearUsuarioUseCase:
    repo: IUserRepository

    def execute(self, user: User) -> User:
        self.repo.guardar(user)
        return user


@dataclass
class ObtenerUsuarioPorIdUseCase:
    repo: IUserRepository

    def execute(self, id: UUID) -> Optional[User]:
        return self.repo.obtener_por_id(id)


@dataclass
class ObtenerUsuarioPorEmailUseCase:
    repo: IUserRepository

    def execute(self, email: str) -> Optional[User]:
        return self.repo.obtener_por_email(email)


@dataclass
class ListarUsuariosPorRolUseCase:
    repo: IUserRepository

    def execute(self, rol: str) -> List[User]:
        return self.repo.obtener_por_rol(rol)
