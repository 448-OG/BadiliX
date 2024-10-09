use common::{MintKeys, Tx, AUTHORITY, USD_MINT};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, signer::Signer, system_instruction};

use spl_token_2022::{extension::ExtensionType, state::Mint};
use spl_token_metadata_interface::state::TokenMetadata;

fn main() {
    let devnet = "https://api.devnet.solana.com".to_string();

    let client = RpcClient::new(devnet);

    //create_poap_mint(&authority, &mint, &client);

    println!("CREATE MINT SIG: {:?}", create_usd_mint(&client));
}

use spl_type_length_value::variable_len_pack::VariableLenPack;

pub(crate) fn create_usd_mint(client: &RpcClient) {
    let mint_keys = MintKeys {
        mint: USD_MINT.pubkey(),
        authority: AUTHORITY.pubkey(),
        update_authority: AUTHORITY.pubkey(),
    };

    // println!(
    //     "Airdrop for Mint Authority Reflected: {}",
    //     ProgramUtils::airdrop(&client, &AUTHORITY.pubkey())
    // );

    let create_mint_ix = create_mint_ixs(&client, &mint_keys);
    let create_poap_mint_sig = Tx::new(&AUTHORITY)
        .add_instructions(create_mint_ix)
        .add_signers(&[&USD_MINT])
        .send_tx(&client);

    println!("POAP CREATE SIG: {:?}", create_poap_mint_sig);
}

pub fn create_mint_ixs(client: &RpcClient, mintkeys: &MintKeys) -> Vec<Instruction> {
    let (key, value) = ("KES DECIMALS".to_string(), "2".to_string());

    let mut metadata = TokenMetadata {
        mint: mintkeys.mint,
        name: "KES/USD".to_string(),
        symbol: "$".to_string(),
        ..Default::default()
    };
    metadata.update_authority.0 = mintkeys.authority;
    metadata
        .additional_metadata
        .push((key.clone(), value.clone()));

    let max_additional_data_bytes = 48u64;

    // Size of MetadataExtension 2 bytes for type, 2 bytes for length
    let metadata_extension_len = 4usize;
    let metadata_extension_bytes_len = metadata.get_packed_len().unwrap();
    let extensions = vec![ExtensionType::MetadataPointer];
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
        6,
    )
    .unwrap();

    let metadata_pointer_ix = spl_token_metadata_interface::instruction::initialize(
        &spl_token_2022::id(),
        &mintkeys.mint,
        &mintkeys.authority,
        &mintkeys.mint,
        &mintkeys.authority,
        "KES/USD".to_string(),
        "$".to_string(),
        String::from("BadiliX-KES-to-USD"),
    );

    let update_metadata_pointer_ix = spl_token_metadata_interface::instruction::update_field(
        &spl_token_2022::id(),
        &mintkeys.mint,
        &mintkeys.authority,
        spl_token_metadata_interface::state::Field::Key(key),
        value,
    );

    vec![
        create_account_ix,
        init_metadata_pointer_ix,
        init_mint_ix,
        metadata_pointer_ix,
        update_metadata_pointer_ix,
    ]
}
