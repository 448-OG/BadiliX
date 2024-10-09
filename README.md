### BadiliX

#### Solana for Dumb Phones

Millions of dumb phones are unable to access the Solana blockchain because major wallets support only smartphones.

BadiliX is an application that allows users to interact with the blockchain without internet using USSD queries.

Users can for example source for USD dollars to make international transfers or create NFTs.

This app demonstrates a user in Kenya sourcing USD by transferring a certain amount of KES to the mint authority offline mobile wallet (could be Airtel Money, Safaricom M-Pesa) and then minting the equivalent onchain based on the exchange rate.

A Key-Deriviation-Function (KDF) is used to mask the phone number of the user by generating a keypair from the phone number of the recipient and a secret key unique to the phone number.
##### Pseudocode
```rust
let kdf = blake3::derive_key(phone_number, random 32 bytes secret key);
let secret_key = ed25519::Secret::from_bytes(kdf);
let public_key = ed25519::PublicKey(secret_key);
let keypair = Keypair::from_bytes(secret_key, public_key);

// Keypair can now sign transactions and the keypair secret can be held in a multi-party computation secure enclave
```


### Running the project
1. The `server` directory is runs the HTTP server responsible for receiving USSD queries and sending SMS messages about transactions to a user's phone number.
    ```sh
    $ cargo run -p server
    ```
2. The `client` directory just initializes the USD mint for the demo to showcase a mint for any currency, in this demo USD
3. `BadiliX-POAP` directory is the frontend where a user can create a event using their wallet and the server will automatically create a mint for the event. The server then listens for USSD requests from a phone number and the creates a Proof-of-Attendance NFT for that phone number.
    ```sh
    $ cd BadiliX-POAP

    $ npx run dev
    ```
    Open browser at [http://localhost:3000/](http://localhost:3000/) to create an event



