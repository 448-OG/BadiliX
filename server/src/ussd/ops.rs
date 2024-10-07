use common::EVENTS_DB;
use percent_encoding::percent_decode;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

use crate::{mint_poap, send_message};

use super::{UssdError, UssdResult};

pub struct EventHandler;

impl EventHandler {
    pub async fn compute_session(info: Ussd, authority: &Keypair, client: &RpcClient) -> String {
        let service_selected = info.text.split("*").collect::<Vec<&str>>();

        let outcome = if service_selected[0].is_empty() {
            Services::None
        } else {
            Services::Poap(info.text.clone())
        };

        let (statemachine, service) = outcome.screens(&info.phone_number, authority, client);

        statemachine.format_state() + service.as_str()
    }
}

#[derive(Debug, Default)]
pub struct Ussd {
    pub(crate) service_code: String,
    pub(crate) phone_number: String,
    pub(crate) text: String,
    pub(crate) session_id: String,
    pub(crate) network_code: String,
}

impl Ussd {
    pub fn new(user_form_data: &str) -> UssdResult<Self> {
        let mut outcome = Self::default();

        let decoded = percent_decode(user_form_data.trim().as_bytes())
            .decode_utf8()
            .unwrap();

        decoded.split("&").try_for_each(|key_value| {
            let key_value = key_value.split("=").collect::<Vec<&str>>();

            let (key, value) = (key_value[0], key_value[1].to_string());

            match key {
                "serviceCode" => outcome.service_code = value,
                "phoneNumber" => outcome.phone_number = value,
                "text" => outcome.text = value,
                "sessionId" => outcome.session_id = value,
                "networkCode" => outcome.network_code = value,
                _ => {
                    return Err(UssdError::UnsupportedFormData {
                        key: key.to_string(),
                        value,
                    })
                }
            }

            Ok(())
        })?;

        Ok(outcome)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub enum UssdStateMachine {
    #[default]
    Start,
    Continue,
    End,
}

impl UssdStateMachine {
    pub fn format_state(&self) -> String {
        match self {
            Self::Start => String::from("CON "),
            Self::Continue => "CON ".to_string(),
            Self::End => "END ".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Services {
    #[default]
    None,
    Poap(String),
}

impl Services {
    pub fn screens(
        &self,
        recipient_phone: &str,
        authority: &Keypair,
        client: &RpcClient,
    ) -> (UssdStateMachine, String) {
        match self {
            Self::None => self.start_screen(),
            Self::Poap(value) => self.poap_screen(value, recipient_phone, authority, client),
        }
    }

    fn start_screen(&self) -> (UssdStateMachine, String) {
        (
            UssdStateMachine::Continue,
            String::new() + "Select a Service \n" + "1. POAP",
        )
    }

    fn poap_screen(
        &self,
        value: &str,
        recipient_phone: &str,
        authority: &Keypair,
        client: &RpcClient,
    ) -> (UssdStateMachine, String) {
        let service_selected = value.split("*").collect::<Vec<&str>>();

        match service_selected.as_slice() {
            ["1"] => (
                UssdStateMachine::Continue,
                String::new() + "Which event do you to attend?",
            ),
            ["1", _] => {
                let mint_name = service_selected.as_slice()[1];
                let mint_name = mint_name.replace("+", " ").to_uppercase();
                if let Some(mint_bytes) = EVENTS_DB.get(&mint_name).unwrap() {
                    let decoded_mint: [u8; 32] = mint_bytes.to_vec().as_slice().try_into().unwrap();
                    let mint_pubkey = solana_program::pubkey::Pubkey::new_from_array(decoded_mint);

                    println!("DECODED MINT PUBKEY: {:?}", &mint_pubkey);

                    let (signature, kdf_key) =
                        mint_poap(recipient_phone, client, authority, mint_pubkey);
                    let signature_uri =
                        format!("https://explorer.solana.com/tx/{signature}?cluster=custom");

                    let mint_name_cloned = mint_name.clone();
                    let kdf_key_cloned = kdf_key.to_string();
                    let recipient_phone = recipient_phone.to_string();

                    let message = format!(
                        "Proof-of-Attendance\nEvent: {mint_name_cloned}\nKDF Key : {kdf_key_cloned}\nSignature: {signature_uri}\n"
                    );

                    println!("MESSAGE:\n {}", &message);

                    smol::spawn(async move {
                        let outcome = send_message(&recipient_phone, &message, "BadiliX").await;

                        dbg!("XXXXXXXXXXXXXXXXXXXXXX");
                        dbg!(&outcome.as_str());
                    })
                    .detach();
                    let outcome = format!("Proof-of-Attendance for {mint_name} Created Successfully!\n Enjoy your event. Cheers!");

                    (UssdStateMachine::End, outcome)
                } else {
                    (
                        UssdStateMachine::End,
                        String::new() + "BadiliX -> Internal Server Error",
                    )
                }
            }

            _ => (UssdStateMachine::End, String::new() + "POAP> Coming soon!"),
        }
    }
}
