//! OpenKey Host Simulator

use openkey_core::core_info;
use openkey_pal::{GpioUserPresenceProvider, RngProvider};

/// Simulador dummy de aleatoriedade em memória para host
struct DummyRng;

impl RngProvider for DummyRng {
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ()> {
        for (i, b) in dest.iter_mut().enumerate() {
            *b = (i % 256) as u8;
        }
        Ok(())
    }
}

/// Simulador de botão de presença de usuário para host
struct DummyUserPresence;

impl GpioUserPresenceProvider for DummyUserPresence {
    fn is_user_present(&mut self) -> bool {
        true
    }
}

fn main() {
    println!("Iniciando OpenKey Software Simulator...");
    println!("Info: {}", core_info());

    let mut rng = DummyRng;
    let mut buf = [0u8; 8];
    if rng.fill_bytes(&mut buf).is_ok() {
        println!("TRNG Emulado: {:?}", buf);
    }

    let mut presence = DummyUserPresence;
    println!("Presença de Usuário: {}", presence.is_user_present());
    println!("Simulador inicializado com sucesso!");
}
