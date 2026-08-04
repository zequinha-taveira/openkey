use openkey_protocols::cbor::CborDecoder;
use openkey_protocols::cose::{encode_protected_header, parse_cose_sign1, CoseAlgorithm};
use openkey_protocols::ctap2::{Ctap2Command, Ctap2Engine, Ctap2Status};
use openkey_protocols::ctap_hid::{
    fragment_payload, CtapHidCommand, CtapHidMessageAssembler, CtapHidPacket,
};
use openkey_protocols::webauthn::{AuthenticatorData, WEBAUTHN_FLAG_UP, WEBAUTHN_FLAG_UV};

#[test]
fn test_end_to_end_ctaphid_ctap2_getinfo_flow() {
    let cid = 0x11223344;
    let aaguid = [0x55; 16];

    // 1. Fragmentar requisição CTAPHID no Host (GetInfo = 0x04)
    let ctap2_cmd_payload = [Ctap2Command::GetInfo.to_u8()];
    let mut req_assembler = CtapHidMessageAssembler::new();
    let mut rx_req_payload_buf = [0u8; 128];
    let mut parsed_req = None;

    fragment_payload(
        cid,
        CtapHidCommand::Cbor.to_u8(),
        &ctap2_cmd_payload,
        |pkt_bytes| {
            let parsed_pkt = CtapHidPacket::parse(pkt_bytes).unwrap();
            if let Some(res) = req_assembler
                .process_packet(&parsed_pkt, &mut rx_req_payload_buf)
                .unwrap()
            {
                parsed_req = Some(res);
            }
        },
    );

    let (res_cid, res_cmd, res_len) = parsed_req.unwrap();
    assert_eq!(res_cid, cid);
    assert_eq!(res_cmd, CtapHidCommand::Cbor.to_u8());
    assert_eq!(res_len, 1);

    // 2. Executar o comando no Ctap2Engine (Firmware Side)
    let mut ctap2_resp_buf = [0u8; 512];
    let ctap2_cmd_byte = rx_req_payload_buf[0];
    let ctap2_payload = &rx_req_payload_buf[1..res_len];

    let resp_len = Ctap2Engine::handle_request(
        ctap2_cmd_byte,
        ctap2_payload,
        aaguid,
        false,
        &mut ctap2_resp_buf,
    );

    assert!(resp_len > 1);
    assert_eq!(ctap2_resp_buf[0], Ctap2Status::Ok.to_u8());

    // 3. Fragmentar a resposta CTAPHID do Firmware para o Host (suporte multi-pacotes)
    let ctap2_resp_payload = &ctap2_resp_buf[..resp_len];
    let mut host_assembler = CtapHidMessageAssembler::new();
    let mut host_rx_buf = [0u8; 512];
    let mut host_parsed_resp = None;

    fragment_payload(
        cid,
        CtapHidCommand::Cbor.to_u8(),
        ctap2_resp_payload,
        |pkt_bytes| {
            let parsed_pkt = CtapHidPacket::parse(pkt_bytes).unwrap();
            if let Some(res) = host_assembler
                .process_packet(&parsed_pkt, &mut host_rx_buf)
                .unwrap()
            {
                host_parsed_resp = Some(res);
            }
        },
    );

    let (h_cid, h_cmd, h_len) = host_parsed_resp.unwrap();
    assert_eq!(h_cid, cid);
    assert_eq!(h_cmd, CtapHidCommand::Cbor.to_u8());
    assert_eq!(h_len, resp_len);
    assert_eq!(host_rx_buf[0], Ctap2Status::Ok.to_u8());

    // 4. Decodificar o mapa CBOR retornado no Host
    let mut dec = CborDecoder::new(&host_rx_buf[1..h_len]);
    let mut key_count = 0;
    dec.decode_map_canonical(0, |entry_dec| {
        key_count += 1;
        let _key = entry_dec.decode_value().unwrap();
        let _val = entry_dec.skip_value_slice().unwrap();
        Ok(())
    })
    .unwrap();

    assert_eq!(key_count, 6); // versions, extensions, aaguid, options, maxMsgSize, pinUvAuthProtocols
}

#[test]
fn test_cose_sign1_and_webauthn_interoperability() {
    let mut prot_buf = [0u8; 16];
    let prot_len = encode_protected_header(CoseAlgorithm::Es256, &mut prot_buf).unwrap();
    let protected_bytes = &prot_buf[..prot_len];

    let auth_data = AuthenticatorData {
        rp_id_hash: [0x33; 32],
        flags: WEBAUTHN_FLAG_UP | WEBAUTHN_FLAG_UV,
        sign_count: 100,
        attested_credential_data: None,
    };

    let mut auth_data_buf = [0u8; 64];
    let auth_data_len = auth_data.serialize(&mut auth_data_buf).unwrap();
    let payload = &auth_data_buf[..auth_data_len];

    let signature = b"test_ecdsa_signature_bytes_64_bytes_length_mock_mock_mock_mock_123456";

    let mut cose_buf = [0u8; 256];
    let cose_len = openkey_protocols::cose::encode_cose_sign1(
        protected_bytes,
        payload,
        signature,
        &mut cose_buf,
    )
    .unwrap();

    let parsed_cose = parse_cose_sign1(&cose_buf[..cose_len]).unwrap();
    assert_eq!(parsed_cose.algorithm, CoseAlgorithm::Es256);
    assert_eq!(parsed_cose.payload, payload);
    assert_eq!(parsed_cose.signature, signature);
}
