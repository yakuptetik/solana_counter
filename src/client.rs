use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_counter::CounterInstructions;

fn main() {
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(rpc_url);

    let payer = Keypair::new();
    let counter_account = Keypair::new();

    // Airdrop some SOL to the payer
    let sig = client
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .unwrap();
    client.confirm_transaction(&sig).unwrap();

    // Create the counter account
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &counter_account.pubkey(),
        client
            .get_minimum_balance_for_rent_exemption(std::mem::size_of::<u32>())
            .unwrap(),
        std::mem::size_of::<u32>() as u64,
        &Pubkey::new_from_array([/* Program ID'nizi buraya ekleyin */]),
    );

    // Increment the counter by 5
    let increment_ix = Instruction::new_with_borsh(
        Pubkey::new_from_array([/* Program ID'nizi buraya ekleyin */]),
        &CounterInstructions::Increment(5),
        vec![AccountMeta::new(counter_account.pubkey(), false)],
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_ix, increment_ix],
        Some(&payer.pubkey()),
        &[&payer, &counter_account],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);

    // Get the counter value
    let account_data = client.get_account_data(&counter_account.pubkey()).unwrap();
    let counter_value = u32::from_le_bytes(account_data[0..4].try_into().unwrap());
    println!("Counter value: {}", counter_value);
}