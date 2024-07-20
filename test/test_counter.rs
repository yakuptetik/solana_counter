use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use solana_counter::{process_instruction, CounterInstructions, CounterAccount};

#[tokio::test]
async fn test_counter_increment() {
    let program_id = Pubkey::new_unique();
    let counter_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "solana_counter",
        program_id,
        processor!(process_instruction),
    );

    program_test.add_account(
        counter_pubkey,
        Account {
            lamports: 5,
            data: vec![0; std::mem::size_of::<CounterAccount>()],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Increment by 5
    let increment_value = 5u32;
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &CounterInstructions::Increment(increment_value),
            vec![AccountMeta::new(counter_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Check the result
    let account = banks_client.get_account(counter_pubkey).await.unwrap().unwrap();
    let counter_account = CounterAccount::try_from_slice(&account.data).unwrap();
    assert_eq!(counter_account.counter, increment_value);
}