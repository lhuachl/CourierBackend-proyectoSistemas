"""
Modelo de persistencia SQLAlchemy para User
"""
from sqlalchemy import Column, String, DateTime, Enum as SQLEnum
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.ext.declarative import declarative_base
import uuid
from datetime import datetime

Base = declarative_base()

class UserDB(Base):
    """Modelo SQLAlchemy para la tabla de usuarios"""
    __tablename__ = "users"
    
    # Campos base
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow, nullable=False)
    
    # Campos específicos de User
    email = Column(String, unique=True, nullable=False, index=True)
    password_hash = Column(String, nullable=False)
    nombre = Column(String, nullable=False)
    apellido = Column(String, nullable=False)
    role = Column(String, nullable=False)
    estado = Column(String, nullable=False, default="activo")
    
    def __repr__(self):
        return f"<UserDB(id={self.id}, email={self.email}, role={self.role})>"
