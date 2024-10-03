use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, system_instruction};
use spl_token_2022::{
    extension::ExtensionType, instruction::initialize_non_transferable_mint, state::Mint,
};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

pub fn create_poap_mint(client: &RpcClient, mintkeys: &MintKeys) -> Vec<Instruction> {
    let metadata_info = mint_metadata();

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

fn mint_metadata() -> MintMetadata {
    let about_us = "Sprint to launch your new crypto startup and compete online to be discovered by the judges, Colosseum's Accelerator, and the Solana community.";

    MintMetadata {
        decimals: 0,
        name: "RADAR HACKATHON".to_string(),
        symbol: "🏅".to_string(),
        uri: "https://www.colosseum.org/radar".to_string(),
        additional_metadata: ("about-us".to_string(), about_us.to_string()),
    }
}

#[derive(Debug)]
pub struct MintKeys {
    pub(crate) mint: Pubkey,
    pub(crate) authority: Pubkey,
    pub(crate) update_authority: Pubkey,
}

#[derive(Debug)]
struct MintMetadata {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub additional_metadata: (String, String),
}
