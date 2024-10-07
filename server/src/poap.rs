use common::{ProgramUtils, RandomBytes, Tx, Utils};
use ed25519_dalek::SecretKey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::instruction::mint_to;

pub fn mint_poap(
    recipient_phone: &str,
    client: &RpcClient,
    authority: &Keypair,
    mint: Pubkey,
) -> (Signature, String) {
    let recipient_key = RandomBytes::<32>::gen();

    let recipient = blake3::keyed_hash(&recipient_key.expose(), recipient_phone.as_bytes());

    let encoded_key = Utils::encode_32byte_dashed(recipient_key.expose_borrowed());

    let decoded = Utils::decode_dashed_bytes(&encoded_key);
    assert_eq!(decoded.as_slice(), recipient_key.expose_borrowed());

    let recipient_secret_key = SecretKey::from_bytes(recipient.as_bytes()).unwrap();
    let recipient_public_key_temp: ed25519_dalek::PublicKey = (&recipient_secret_key).into();
    let recipient_public_key = Pubkey::new_from_array(*recipient_public_key_temp.as_bytes());
    let mut recipient_keypair_bytes = Vec::<u8>::new();
    recipient_keypair_bytes.extend_from_slice(recipient_secret_key.as_bytes());
    recipient_keypair_bytes.extend_from_slice(&recipient_public_key.to_bytes());
    let recipient_keypair = Keypair::from_bytes(&recipient_keypair_bytes).unwrap();

    println!("RECIPIENT: {:?}", recipient_keypair.pubkey());

    let recipient_ata = get_associated_token_address_with_program_id(
        &recipient_keypair.pubkey(),
        &mint,
        &spl_token_2022::id(),
    );

    println!("RECIPIENT ATA: {:?}", recipient_ata);

    ProgramUtils::airdrop(client, &recipient_keypair.pubkey());

    let create_ata_ix = create_associated_token_account(
        &authority.pubkey(),
        &recipient_keypair.pubkey(),
        &mint,
        &spl_token_2022::id(),
    );

    let mint_to_ix = mint_to(
        &spl_token_2022::id(),
        &mint,
        &recipient_ata,
        &authority.pubkey(),
        &[&authority.pubkey()],
        1,
    )
    .unwrap();

    let signature = Tx::new(&authority)
        .add_signer(authority)
        .add_instructions(vec![create_ata_ix, mint_to_ix])
        .send_tx(client);

    (signature, encoded_key)
}
