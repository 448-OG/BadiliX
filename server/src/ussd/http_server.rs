use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::{EventHandler, Ussd};

pub(crate) fn start_server() {
    trillium_smol::run(|mut conn: trillium::Conn| async move {
        let authority = Keypair::from_bytes(&[
            221, 138, 79, 165, 198, 65, 148, 172, 141, 152, 228, 17, 124, 124, 229, 80, 26, 128,
            236, 105, 94, 119, 134, 201, 5, 32, 90, 213, 9, 116, 172, 168, 93, 88, 48, 22, 130, 79,
            230, 210, 105, 156, 125, 206, 40, 13, 0, 220, 89, 187, 94, 220, 61, 135, 160, 193, 210,
            247, 221, 198, 221, 142, 56, 86,
        ])
        .unwrap();

        println!("AUTHORITY: {:?}", authority.pubkey());

        let localhost = "http://localhost:8899".to_string();
        let client = RpcClient::new(localhost);
        let user_data = conn.request_body().await.read_string().await;

        if user_data.is_err() {
            return conn.ok("END Internal Server Error");
        }

        let user_data = user_data.unwrap();
        println!("{:?}", &user_data);

        let decoded = Ussd::new(&user_data);

        if decoded.is_err() {
            return conn.ok("END Internal Server Error");
        }

        let decoded = decoded.unwrap();

        println!("{:?}", decoded);

        let body = EventHandler::compute_session(decoded, &authority, &client).await;

        println!("response - {:?}", &body);

        conn.ok(body)
    });
}
