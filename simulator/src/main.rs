use openkey_core::Authenticator;
use openkey_usb::Transport;

#[derive(Default)]
struct MemoryTransport {
    incoming: Vec<u8>,
    outgoing: Vec<u8>,
}

impl Transport for MemoryTransport {
    type Error = ();

    fn receive(&mut self, packet: &mut [u8]) -> Result<usize, Self::Error> {
        let size = self.incoming.len().min(packet.len());
        packet[..size].copy_from_slice(&self.incoming[..size]);
        self.incoming.drain(..size);
        Ok(size)
    }

    fn send(&mut self, packet: &[u8]) -> Result<(), Self::Error> {
        self.outgoing.extend_from_slice(packet);
        Ok(())
    }
}

fn main() {
    let mut authenticator = Authenticator::new();
    authenticator
        .provision()
        .expect("simulador deve provisionar");
    let mut transport = MemoryTransport::default();
    transport.send(b"get-info").expect("transporte em memória");
    println!(
        "openkey-simulator ready={}",
        authenticator.state().is_ready()
    );
}

#[cfg(test)]
mod tests {
    use super::{MemoryTransport, Transport};

    #[test]
    fn transporte_preserva_pacote() {
        let mut transport = MemoryTransport {
            incoming: vec![1, 2, 3],
            outgoing: Vec::new(),
        };
        let mut packet = [0; 3];
        assert_eq!(transport.receive(&mut packet), Ok(3));
        assert_eq!(packet, [1, 2, 3]);
        transport.send(&packet).unwrap();
        assert_eq!(transport.outgoing, [1, 2, 3]);
    }
}
