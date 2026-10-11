#!/usr/bin/env sh
set -eu

# O núcleo reutilizável não pode importar HALs, registradores ou nomes de placas.
if grep -RInE --include='*.rs' --include='Cargo.toml' \
  'rp23xx|rp2350|stm32|nrf[0-9]*|esp32|embedded_hal|board-specific|boards/' \
  crates >/dev/null 2>&1; then
  echo "architecture error: reusable crates reference a board or MCU-specific implementation" >&2
  exit 1
fi

echo "MCU independence: OK"
