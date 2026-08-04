---
layout: pid
title: OpenKey Security Key
owner: openkey-team
license: MIT
site: https://github.com/zequinha-taveira/openkey
source: https://github.com/zequinha-taveira/openkey
---
OpenKey is a universal FIDO2/WebAuthn hardware security key framework
that runs on existing development boards (Raspberry Pi Pico 2, XIAO RP2350,
Tiny2350, etc.) via data-driven board profiles. The firmware implements
CTAP2.1, CBOR, COSE, WebAuthn and USB HID transport in memory-safe Rust.
PID 0x4F4B ("OK") is reserved for custom hardware that integrates the
OpenKey firmware.
