// Mark this test as BPF-only due to current `ProgramTest` limitations when
// CPIing into the system program
#![cfg(feature = "test-sbf")]

use {
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
    spl_math_example::{id, instruction, processor::process_instruction},
};

#[tokio::test]
async fn test_precise_sqrt_u64_max() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::precise_sqrt(u64::MAX)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 358721);
}

#[tokio::test]
async fn test_precise_sqrt_u32_max() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::precise_sqrt(u32::MAX as u64)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 181140);
}

#[tokio::test]
async fn test_sqrt_u64() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction =
        Transaction::new_with_payer(&[instruction::sqrt_u64(u64::MAX)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 1829);
}

#[tokio::test]
async fn test_sqrt_u128() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::sqrt_u128(u64::MAX as u128)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 3890);
}

#[tokio::test]
async fn test_sqrt_u128_max() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction =
        Transaction::new_with_payer(&[instruction::sqrt_u128(u128::MAX)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 6743);
}

#[tokio::test]
async fn test_u64_multiply() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction =
        Transaction::new_with_payer(&[instruction::u64_multiply(42, 84)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 980);
}

#[tokio::test]
async fn test_u64_divide() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction =
        Transaction::new_with_payer(&[instruction::u64_divide(3, 1)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 959);
}

#[tokio::test]
async fn test_f32_multiply() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::f32_multiply(1.5_f32, 2.0_f32)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 1156);
}

#[tokio::test]
async fn test_f32_divide() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::f32_divide(3_f32, 1.5_f32)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 1206);
}

#[tokio::test]
async fn test_f32_exponentiate() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::f32_exponentiate(4_f32, 2_f32)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 1199);
}

#[tokio::test]
async fn test_f32_natural_log() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::f32_natural_log(1_f32.exp())],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 3019);
}

#[tokio::test]
async fn test_f32_normal_cdf() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction =
        Transaction::new_with_payer(&[instruction::f32_normal_cdf(0_f32)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 2523);
}

#[tokio::test]
async fn test_f64_pow() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::f64_pow(50_f64, 10.5_f64)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 12909);
}

#[tokio::test]
async fn test_u128_multiply() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::u128_multiply(u64::MAX.into(), u64::MAX.into())],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_u128_divide() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::u128_divide(u128::MAX, u128::MAX / 69)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 1415);
}

#[tokio::test]
async fn test_f64_multiply() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::f64_multiply(f64::powf(2., 42.), 1e-4)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 1267);
}

#[tokio::test]
async fn test_f64_divide() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::f64_divide(f64::powf(2., 42.), 420420.6969)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 1332);
}

#[tokio::test]
async fn test_noop() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));

    pc.set_compute_max_units(1_000_000);

    let (banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction =
        Transaction::new_with_payer(&[instruction::noop()], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction_with_metadata(transaction).await.unwrap();
    assert_eq!(result.metadata.unwrap().compute_units_consumed, 361);
}
