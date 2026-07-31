"""Módulo de Transporte CTAP HID (FIDO CTAP2 Spec Section 11)"""

import struct
from typing import Tuple, List, Optional
from openkey.exceptions import TransportError

CTAPHID_PACKET_SIZE = 64
CTAPHID_BROADCAST_CID = 0xFFFFFFFF
CTAPHID_CMD_BIT = 0x80

CMD_PING = 0x81
CMD_MSG = 0x83
CMD_INIT = 0x86
CMD_WINK = 0x88
CMD_CBOR = 0x90
CMD_CANCEL = 0x91
CMD_KEEPALIVE = 0xBB
CMD_ERROR = 0xBF

class CtapHidPacket:
    """Serializador e deserializador de pacotes USB HID de 64 bytes"""

    @staticmethod
    def create_init_packet(cid: int, cmd: int, payload: bytes) -> bytes:
        """Cria o primeiro pacote (Initialization Packet)"""
        cmd_byte = cmd if (cmd & CTAPHID_CMD_BIT) else (cmd | CTAPHID_CMD_BIT)
        payload_len = len(payload)
        header = struct.pack(">IBH", cid, cmd_byte, payload_len)
        data = payload[:57]
        packet = header + data
        return packet.ljust(CTAPHID_PACKET_SIZE, b"\x00")

    @staticmethod
    def create_cont_packet(cid: int, seq: int, payload: bytes) -> bytes:
        """Cria um pacote de continuação (Continuation Packet)"""
        seq_byte = seq & 0x7F
        header = struct.pack(">IB", cid, seq_byte)
        data = payload[:59]
        packet = header + data
        return packet.ljust(CTAPHID_PACKET_SIZE, b"\x00")

    @staticmethod
    def parse_packet(data: bytes) -> Tuple[int, int, bytes, Optional[int]]:
        """Parseia um pacote de 64 bytes. Retorna (cid, cmd_or_seq, payload, total_len_if_init)"""
        if len(data) < CTAPHID_PACKET_SIZE:
            raise TransportError("Tamanho de pacote inválido (< 64 bytes)")

        cid, cmd_or_seq = struct.unpack(">IB", data[:5])
        if cmd_or_seq & CTAPHID_CMD_BIT:
            # Init packet
            payload_len = struct.unpack(">H", data[5:7])[0]
            payload = data[7:7 + min(payload_len, 57)]
            return (cid, cmd_or_seq, payload, payload_len)
        else:
            # Cont packet
            seq = cmd_or_seq & 0x7F
            payload = data[5:]
            return (cid, seq, payload, None)

class CtapHidMessageAssembler:
    """Montador e fragmentador de mensagens CTAPHID"""

    @staticmethod
    def fragment_message(cid: int, cmd: int, payload: bytes) -> List[bytes]:
        """Divide uma mensagem em um ou mais pacotes CTAPHID de 64 bytes"""
        packets = []
        # Init packet
        init_pkt = CtapHidPacket.create_init_packet(cid, cmd, payload)
        packets.append(init_pkt)

        remaining = payload[57:]
        seq = 0
        while remaining:
            chunk = remaining[:59]
            remaining = remaining[59:]
            cont_pkt = CtapHidPacket.create_cont_packet(cid, seq, chunk)
            packets.append(cont_pkt)
            seq += 1

        return packets

    @staticmethod
    def assemble_message(packets: List[bytes]) -> Tuple[int, int, bytes]:
        """Recompõe os pacotes recebidos em um payload completo. Retorna (cid, cmd, payload)"""
        if not packets:
            raise TransportError("Lista de pacotes vazia")

        cid, cmd, payload, total_len = CtapHidPacket.parse_packet(packets[0])
        if total_len is None:
            raise TransportError("Primeiro pacote deve ser um pacote Init")

        received = bytearray(payload)
        expected_seq = 0

        for pkt in packets[1:]:
            if len(received) >= total_len:
                break
            p_cid, p_seq, p_data, _ = CtapHidPacket.parse_packet(pkt)
            if p_cid != cid:
                raise TransportError("CID inconsistente entre pacotes")
            if p_seq != expected_seq:
                raise TransportError(f"Sequência incorreta: esperado {expected_seq}, recebido {p_seq}")
            expected_seq += 1
            needed = total_len - len(received)
            received.extend(p_data[:needed])

        return (cid, cmd, bytes(received[:total_len]))
