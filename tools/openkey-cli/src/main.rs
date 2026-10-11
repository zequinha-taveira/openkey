fn main() {
    let authenticator = openkey_core::Authenticator::new();
    println!("openkey-cli: estado inicial = {:?}", authenticator.state());
}
