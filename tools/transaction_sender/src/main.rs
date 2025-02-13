use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{Keypair, Signer}, signer::EncodableKey, transaction::Transaction
};

const PROGRAM_ID_STR: &str = "CAgzhmC3vfukCmZQgSd5jhZBUdLujd3hoLZztsJhjfD9";

fn main() {
    let url: String = "http://127.0.0.1:8899".to_string();
    let client: RpcClient = RpcClient::new(url);

    let program_id: Pubkey = Pubkey::from_str(PROGRAM_ID_STR).unwrap();

    let payer_result = solana_sdk::signature::read_keypair_file("/home/brandonneway/.config/solana/id.json");
    
    let payer = match payer_result {
        Ok(keypair) => keypair,
        Err(error) => {
            eprintln!("Error reading keypair: {}", error);
            return;
        }
    };

    let mut get_balance_result: Result<u64, solana_client::client_error::ClientError> = client.get_balance(&payer.pubkey());

    let mut balance: u64 = get_balance_result.unwrap();
    println!("Current Balance: {}", balance);

    // Airdrop 1 SOL to the payer
    let airdrop_signature: solana_sdk::signature::Signature = client.request_airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    let airdrop_confirmation: Result<bool, solana_client::client_error::ClientError> = client.confirm_transaction(&airdrop_signature);
    match airdrop_confirmation {
        Ok(_asdf) => {
            println!("Airdrop confirmation success");
        },
        Err(errorino) => {
            eprintln!("Airdrop confirmation error: {}", errorino);
            return;
        }
    }

    get_balance_result = client.get_balance(&payer.pubkey());

    balance = get_balance_result.unwrap();
    println!("Current Balance: {}", balance);

    let word: [u8; 32] = *b"Here is some sample data\0\0\0\0\0\0\0\0";

    // Create an instruction (modify as per your program's requirements)
    let instruction: Instruction = Instruction::new_with_bytes(
        program_id,
        &word, // Instruction datap
        vec![AccountMeta::new(payer.pubkey(), true)],
    );

    // Create and send the transaction
    let recent_blockhash: solana_sdk::hash::Hash = client.get_latest_blockhash().unwrap();
    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let signature: solana_sdk::signature::Signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
    return;
}