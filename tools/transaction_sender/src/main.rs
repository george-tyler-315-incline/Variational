use std::env;
use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};

fn main() {
    // Expect at least one argument: the on-chain program ID.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <program-id> [input...]", args[0]);
        std::process::exit(1);
    }

    // Parse the program ID from the first argument.
    let program_id_str = &args[1];
    let program_id = match Pubkey::from_str(program_id_str) {
        Ok(pid) => pid,
        Err(e) => {
            eprintln!("Invalid program id: {}", e);
            std::process::exit(1);
        }
    };

    // Encode additional inputs (if any) into a byte vector.
    let mut instruction_data = Vec::new();

    // For each argument after the program ID...
    for input in args.iter().skip(2) {
        // Try to parse as an integer.
        if let Ok(num) = input.parse::<i64>() {
            // Type marker 0 for integer.
            instruction_data.push(0u8);
            instruction_data.extend_from_slice(&num.to_le_bytes());
        } else {
            // Type marker 1 for string.
            instruction_data.push(1u8);
            let bytes = input.as_bytes();
            let len = bytes.len();
            if len > 255 {
                eprintln!("String too long (max 255 bytes): {}", input);
                std::process::exit(1);
            }
            // Store the length as one byte.
            instruction_data.push(len as u8);
            instruction_data.extend_from_slice(bytes);
        }
    }

    // If no extra input was provided, use a default message.
    if instruction_data.is_empty() {
        instruction_data.extend_from_slice(b"default sample data");
    }

    // Set up the RPC client.
    let url = "http://127.0.0.1:8899".to_string();
    let client = RpcClient::new(url);

    // Read the payer keypair.
    let payer_path = "/home/brandonneway/.config/solana/id.json";
    let payer: Keypair = read_keypair_file(payer_path)
        .unwrap_or_else(|err| panic!("Failed to read keypair from {}: {}", payer_path, err));

    // Display current balance.
    let balance = client.get_balance(&payer.pubkey()).unwrap();
    println!("Current Balance: {}", balance);

    // Airdrop 10 SOL (10_000_000_000 lamports) to the payer.
    let airdrop_signature = client.request_airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    client.confirm_transaction(&airdrop_signature).unwrap();

    let balance = client.get_balance(&payer.pubkey()).unwrap();
    println!("Current Balance after airdrop: {}", balance);

    // Create the instruction with the provided input data.
    let instruction = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![AccountMeta::new(payer.pubkey(), true)],
    );

    // Build and send the transaction.
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}
