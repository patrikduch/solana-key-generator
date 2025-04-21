use std::error::Error as StdError;
use bip39::Mnemonic;
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
};
use rand::{rngs::OsRng, RngCore};
use bs58; // Add base58 encoding

fn main() -> Result<(), Box<dyn StdError>> {
    // 1. Generate a random Solana keypair directly
    let keypair = Keypair::new();
    
    // 2. Generate entropy for mnemonic (16 bytes = 128 bits = 12 words)
    let mut entropy = [0u8; 16];
    OsRng.fill_bytes(&mut entropy);
    
    // 3. Create mnemonic from entropy
    let mnemonic = Mnemonic::from_entropy(&entropy)?;
    
    // 4. Convert mnemonic to string
    let seed_phrase = mnemonic.to_string();

    // 5. Get the full keypair bytes
    let keypair_bytes = keypair.to_bytes();
    
    // 6. Convert private key to base58 encoding (the format used by Solana)
    let private_key_base58 = bs58::encode(&keypair_bytes).into_string();

    // 7. Display wallet information
    println!("Solana Wallet Information:");
    println!("=========================");
    println!("Mnemonic:      {}", seed_phrase);
    println!("Public Key:    {}", keypair.pubkey());
    println!("Private Key:   {}", private_key_base58);
    
    // 8. Additional info about the keys
    println!("\nKey Details:");
    println!("Public Key (base58):  {}", keypair.pubkey().to_string());
    println!("Public Key (bytes):   {:?}", keypair.pubkey().to_bytes());
    println!("Private Key (bytes):  {:?}", &keypair_bytes[..32]);
    
    println!("\nNote: In this implementation, the keypair and mnemonic are generated");
    println!("independently. To create a wallet that can be recovered from the mnemonic,");
    println!("you would need to implement BIP44/BIP32 derivation paths.");

    Ok(())
}