class clientes(StandartModel):#no se usa id por que es un campo heredado de StandartModel
    id_usuario : UUID #FK index 
    tipo : TipoClienteEnum #normal,empresa /index
    documento : str = Field(..., description="Client document number")# index
    name : str = Field(..., description="Client name")
    firstLastName : Optional[str] = Field(None, description="Client first last name")
    secondLastName : Optional[str] = Field(None, description="Client second last name")
    telefono : Optional[str] = Field(None, description="Client phone number")
    direccion : Optional[str] = Field(None, description="Client address")
    