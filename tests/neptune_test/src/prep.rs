#![allow(dead_code)]

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{account::Account, pubkey},
};
use anchor_lang::prelude::{Clock, Pubkey};
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{self, TokenAccount},
};
use anyhow::{Ok, Result};
use litesvm::LiteSVM;
use spl_token::{
    solana_program::{program_option::COption, program_pack::Pack},
    state::{Account as SplTokenAccount, AccountState},
    ui_amount_to_amount,
};

use crate::neptune;

pub fn load_accounts(svm: &mut LiteSVM) -> Result<(Pubkey, Pubkey, Pubkey, Pubkey)> {
    // load neptune program
    let neptune_program_id = neptune::ID;
    let neptune_bin = include_bytes!("../../../target/deploy/neptune.so");
    svm.add_program(neptune_program_id, neptune_bin);

    // load locked_voter program
    let locked_voter_program_id = pubkey!("voTpe3tHQ7AjQHMapgSue2HJFAh2cGsdokqN3XqmVSj");
    let locked_voter_bin = include_bytes!("../../../.account_bytes/programs/locked_voter.so");
    svm.add_program(locked_voter_program_id, locked_voter_bin);

    // load jup mint
    let jup_mint_pk = pubkey!("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN");
    let jup_mint_account = include_bytes!(
        "../../../.account_bytes/accounts/JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"
    );
    svm.set_account(
        jup_mint_pk,
        Account {
            lamports: 1_000_000_000,
            data: jup_mint_account.to_vec(),
            owner: token::ID,
            executable: false,
            rent_epoch: 0,
        },
    )?;

    // load locker
    let locker_pk = pubkey!("CVMdMd79no569tjc5Sq7kzz8isbfCcFyBS5TLGsrZ5dN");
    let locker_account_bytes = include_bytes!(
        "../../../.account_bytes/accounts/CVMdMd79no569tjc5Sq7kzz8isbfCcFyBS5TLGsrZ5dN"
    );
    svm.set_account(
        locker_pk,
        Account {
            lamports: 1_000_000_000,
            data: locker_account_bytes.to_vec(),
            owner: locked_voter_program_id,
            executable: false,
            rent_epoch: 0,
        },
    )?;

    Ok((
        locked_voter_program_id,
        neptune_program_id,
        locker_pk,
        jup_mint_pk,
    ))
}

pub fn sync_clock(svm: &mut LiteSVM, rpc: &RpcClient) -> Result<i64> {
    let slot = rpc.get_slot()?;
    let timestamp = rpc.get_block_time(slot)?;

    let mut initial_clock = svm.get_sysvar::<Clock>();
    initial_clock.unix_timestamp = timestamp;
    svm.set_sysvar::<Clock>(&initial_clock);

    Ok(timestamp)
}

pub fn write_token_account(
    svm: &mut LiteSVM,
    owner: &Pubkey,
    mint: &Pubkey,
    decimals: u8,
) -> Result<Pubkey> {
    let owner_ata = get_associated_token_address(&owner, &mint);
    let token_acc = SplTokenAccount {
        mint: mint.clone(),
        owner: owner.clone(),
        amount: ui_amount_to_amount(500_000_f64, decimals),
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    let mut acc_bytes = [0u8; TokenAccount::LEN];
    SplTokenAccount::pack(token_acc, &mut acc_bytes).unwrap();
    svm.set_account(
        owner_ata,
        Account {
            lamports: 1_000_000_000,
            data: acc_bytes.to_vec(),
            owner: token::ID,
            executable: false,
            rent_epoch: 0,
        },
    )?;

    Ok(owner_ata)
}
