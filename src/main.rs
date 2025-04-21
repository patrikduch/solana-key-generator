use reqwest::Error;
use serde_json::Value;

use solana_sdk::{
    signature::{Keypair, Signer},
};

#[tokio::main]
async fn main() -> Result<(), Error> {

       // Generate a new Solana keypair
       let keypair = Keypair::new();
    
       // Get public key (base58 string)
       let public_key = keypair.pubkey().to_string();
       
       // Get private key (base58-encoded secret key)
       let private_key = keypair.to_base58_string();
   
       println!("Generated Solana Key Pair:");
       println!("Public Key:  {}", public_key);
       println!("Private Key: {}", private_key);
 

    Ok(())
}