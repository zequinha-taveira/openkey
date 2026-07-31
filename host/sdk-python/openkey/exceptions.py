"""Exceções do OpenKey SDK"""

class OpenKeyError(Exception):
    """Exceção base do OpenKey SDK"""
    pass

class TransportError(OpenKeyError):
    """Erro de comunicação no transporte CTAPHID ou USB"""
    pass

class CtapError(OpenKeyError):
    """Erro retornado pelo dispositivo CTAP2"""
    def __init__(self, status_code: int, message: str = ""):
        self.status_code = status_code
        self.message = message or f"CTAP2 Error 0x{status_code:02x}"
        super().__init__(self.message)
