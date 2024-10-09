use std::str::FromStr;

use common::{RecipientInfo, Tx, Utils, AUTHORITY};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, signer::keypair::Keypair, system_instruction};
use solana_sdk::{pubkey::Pubkey, signature::Signature, signer::Signer};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::instruction::mint_to;
use spl_token_2022::{
    extension::ExtensionType, instruction::initialize_non_transferable_mint, state::Mint,
};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

use common::{MintKeys, MintMetadata, EVENTS_DB};

use crate::FormHandler;

pub(crate) fn create_poap_mint(
    client: &RpcClient,
    form_data: FormHandler,
) -> Result<Signature, String> {
    let form_data_cloned = form_data.clone();

    let mint_keypair = Keypair::new();
    let update_authority = match Pubkey::from_str(form_data.upgrade_authority.as_str()) {
        Ok(value) => value,
        Err(_) => return Err("Invalid Public Key".to_string()),
    };

    let user_signature_bytes = bs58::decode(form_data.signature)
        .into_vec()
        .map_err(|_| "Invalid Base58 Signature".to_string())?;

    let user_signature = ed25519_dalek::Signature::from_bytes(&user_signature_bytes)
        .map_err(|_| "Invalid Signature Bytes".to_string())?;

    let verifying_pubkey_bytes = bs58::decode(&form_data.upgrade_authority)
        .into_vec()
        .map_err(|_| "Invalid Base58 Signature".to_string())?;

    let verifying_key = ed25519_dalek::PublicKey::from_bytes(&verifying_pubkey_bytes)
        .map_err(|_| "Invalid Public Key".to_string())?;

    use ed25519_dalek::Verifier;

    if verifying_key
        .verify(form_data.mint_name.as_bytes(), &user_signature)
        .is_err()
    {
        return Err("Invalid Signature.".to_string());
    }

    let mint_keys = MintKeys {
        mint: mint_keypair.pubkey(),
        authority: AUTHORITY.pubkey(),
        update_authority,
    };

    EVENTS_DB
        .insert(form_data.mint_name, &mint_keypair.pubkey().to_bytes())
        .unwrap();

    let poap_ixs = create_poap_mint_ixs(&client, &mint_keys, &form_data_cloned);
    let create_poap_mint_sig = Tx::new(&AUTHORITY)
        .add_instructions(poap_ixs)
        .add_signers(&[&AUTHORITY, &mint_keypair])
        .send_tx(&client);

    println!("POAP CREATE SIG: {:?}", create_poap_mint_sig);

    Ok(create_poap_mint_sig)
}

pub fn create_poap_mint_ixs(
    client: &RpcClient,
    mintkeys: &MintKeys,
    form_data: &FormHandler,
) -> Vec<Instruction> {
    let metadata_info = mint_metadata(form_data);

    let mut metadata = TokenMetadata {
        mint: mintkeys.mint,
        name: metadata_info.name.clone(),
        symbol: metadata_info.symbol.clone(),
        uri: metadata_info.uri.clone(),
        ..Default::default()
    };
    metadata.update_authority.0 = mintkeys.authority;
    metadata.additional_metadata.push((
        metadata_info.additional_metadata.0.clone(),
        metadata_info.additional_metadata.1.clone(),
    ));

    let max_additional_data_bytes = 48u64;

    // Size of MetadataExtension 2 bytes for type, 2 bytes for length
    let metadata_extension_len = 4usize;
    let metadata_extension_bytes_len = metadata.get_packed_len().unwrap();
    let extensions = vec![
        ExtensionType::NonTransferable,
        ExtensionType::MetadataPointer,
    ];
    let mint_len = ExtensionType::try_calculate_account_len::<Mint>(&extensions).unwrap();
    let mut rent_for_extensions = client
        .get_minimum_balance_for_rent_exemption(
            mint_len + metadata_extension_len + metadata_extension_bytes_len,
        )
        .unwrap();
    // Ensure enough space can be allocated for the additional info
    rent_for_extensions += rent_for_extensions + max_additional_data_bytes;

    let create_account_ix = system_instruction::create_account(
        &&mintkeys.authority,
        &mintkeys.mint,
        rent_for_extensions,
        mint_len as u64,
        &spl_token_2022::id(),
    );

    let non_transferable_ix =
        initialize_non_transferable_mint(&spl_token_2022::id(), &mintkeys.mint).unwrap();

    // Initialize metadata pointer extension
    let init_metadata_pointer_ix =
        spl_token_2022::extension::metadata_pointer::instruction::initialize(
            &spl_token_2022::id(),
            &mintkeys.mint,
            Some(mintkeys.authority),
            Some(mintkeys.mint),
        )
        .unwrap();

    // Initialize the Mint Account data
    let init_mint_ix = spl_token_2022::instruction::initialize_mint(
        &spl_token_2022::id(),
        &mintkeys.mint,
        &mintkeys.authority,
        Some(&mintkeys.authority),
        metadata_info.decimals,
    )
    .unwrap();

    let metadata_pointer_ix = spl_token_metadata_interface::instruction::initialize(
        &spl_token_2022::id(),
        &mintkeys.mint,
        &mintkeys.authority,
        &mintkeys.mint,
        &mintkeys.authority,
        metadata_info.name,
        metadata_info.symbol,
        metadata_info.uri,
    );

    let update_metadata_pointer_ix = spl_token_metadata_interface::instruction::update_field(
        &spl_token_2022::id(),
        &mintkeys.mint,
        &mintkeys.authority,
        spl_token_metadata_interface::state::Field::Key(metadata_info.additional_metadata.0),
        metadata_info.additional_metadata.1,
    );

    vec![
        create_account_ix,
        non_transferable_ix,
        init_metadata_pointer_ix,
        init_mint_ix,
        metadata_pointer_ix,
        update_metadata_pointer_ix,
    ]
}

fn mint_metadata(form_data: &FormHandler) -> MintMetadata {
    MintMetadata {
        decimals: 0,
        name: form_data.mint_name.clone(),
        symbol: form_data.symbol.clone(),
        uri: form_data.uri.clone(),
        additional_metadata: ("about-us".to_string(), form_data.about_us.clone()),
    }
}

pub fn mint_poap(recipient_phone: &str, client: &RpcClient, mint: Pubkey) -> (Signature, String) {
    let recipient_info = RecipientInfo::new(recipient_phone);
    let (temp_kdf, recipient_info) = recipient_info.temp_keypair(mint);

    let recipient_pubkey = recipient_info.pubkey();

    println!("RECIPIENT: {:?}", recipient_pubkey);

    let recipient_ata = get_associated_token_address_with_program_id(
        &recipient_pubkey,
        &mint,
        &spl_token_2022::id(),
    );

    println!("RECIPIENT ATA: {:?}", recipient_ata);

    let create_ata_ix = create_associated_token_account(
        &AUTHORITY.pubkey(),
        &recipient_pubkey,
        &mint,
        &spl_token_2022::id(),
    );

    let mint_to_ix = mint_to(
        &spl_token_2022::id(),
        &mint,
        &recipient_ata,
        &AUTHORITY.pubkey(),
        &[&AUTHORITY.pubkey()],
        1,
    )
    .unwrap();

    let signature = Tx::new(&AUTHORITY)
        .add_instructions(vec![create_ata_ix, mint_to_ix])
        .send_tx(client);

    let encoded_key = Utils::encode_32byte_dashed(temp_kdf.as_bytes());

    (signature, encoded_key)
}
