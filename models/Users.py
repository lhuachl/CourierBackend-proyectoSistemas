from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
class User(StandartModel):
    #id : UUID index
    email : str = Field(..., description="User email address")# index
    name :  str = Field(..., description="User name")
    firstLastName : Optional[str] = Field(None, description="User first last name")
    secondLastName : Optional[str] = Field(None, description="User second last name")
    role : RoleEnum #admin,operador,cliente,trasportista /index
    estado : EstadoEnum #activo,inactivo,suspendido

