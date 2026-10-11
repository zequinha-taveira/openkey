use openkey_host::{DeviceClient, OfflineTransport, ProvisioningRequest, UpdatePackage};

fn main() {
    let command = std::env::args().nth(1).unwrap_or_else(|| "info".to_owned());
    let mut client = DeviceClient::new(OfflineTransport::default());

    let result = match command.as_str() {
        "info" => client.get_info().map(|info| {
            println!(
                "protocol={} firmware={} provisioned={}",
                info.protocol_version, info.firmware_version, info.provisioned
            );
        }),
        "provision" => client
            .provision(ProvisioningRequest {
                label: "openkey-cli",
            })
            .map(|_| println!("provisioned=true")),
        "update" => client
            .update(UpdatePackage {
                version: 1,
                image: b"offline-demo-image",
                signature_valid: true,
            })
            .map(|_| println!("updated=true")),
        "diagnostics" => client.diagnostics().map(|diagnostics| {
            println!(
                "transport_ok={} storage_ok={} firmware={}",
                diagnostics.transport_ok, diagnostics.storage_ok, diagnostics.firmware_version
            );
        }),
        "--version" | "version" => {
            println!("openkey-cli {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        _ => {
            eprintln!("uso: openkey-cli [info|provision|update|diagnostics]");
            return;
        }
    };

    if let Err(error) = result {
        eprintln!("openkey-cli error: {error:?}");
        std::process::exit(1);
    }
}
