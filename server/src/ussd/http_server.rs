use common::{ProgramUtils, AUTHORITY};
use serde::{Deserialize, Serialize};
use solana_sdk::signer::Signer;
use trillium::Conn;
use trillium_logger::Logger;
use trillium_router::Router;

use crate::{create_poap_mint, EventHandler, Ussd, CLIENT};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FormHandler {
    pub(crate) mint_name: String,
    pub(crate) upgrade_authority: String,
    pub(crate) signature: String,
    pub(crate) name: String,
    pub(crate) symbol: String,
    pub(crate) uri: String,
    pub(crate) about_us: String,
}

pub(crate) fn start_server() {
    trillium_smol::run((
        Logger::new(),
        handle_poap_options,
        Router::new()
            .post("/", handle_root)
            .post("/poap", handle_poap),
    ));
}

async fn handle_poap_options(conn: Conn) -> Conn {
    conn.with_response_header("Access-Control-Allow-Origin", "*")
        .with_response_header("Access-Control-Allow-Headers", "content-type")
}

async fn handle_root(mut conn: Conn) -> Conn {
    println!("AUTHORITY: {:?}", AUTHORITY.pubkey());
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

    let body = EventHandler::compute_session(decoded).await;

    println!("response - {:?}", &body);

    conn.ok(body)
}

async fn handle_poap(mut conn: Conn) -> Conn {
    let mint_name_json = conn
        .request_body()
        .await
        .read_string()
        .await
        .unwrap_or("Encountered_error".to_string());

    if let Ok(form) = serde_json::from_str::<FormHandler>(&mint_name_json) {
        match create_poap_mint(&CLIENT, form) {
            Ok(sig) => {
                let body = ProgramUtils::signature_as_link(&sig);

                conn.ok(body)
            }
            Err(error) => {
                let error_msg =
                    String::new() + "Encountered Error while creating mint - " + error.as_str();

                conn.with_status(500).ok(error_msg)
            }
        }
    } else {
        println!("serde error");

        conn.with_status(500)
            .with_body("Encountered a server error")
    }
}
