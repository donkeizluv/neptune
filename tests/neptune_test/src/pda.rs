#![allow(dead_code)]

use anchor_lang::prelude::Pubkey;

use crate::neptune;

pub fn find_vault_pda(locker_pk: &Pubkey, player_pk: &Pubkey) -> Pubkey {
    let (vault, _) = Pubkey::find_program_address(
        &[b"vault".as_ref(), locker_pk.as_ref(), player_pk.as_ref()],
        &neptune::ID,
    );

    vault
}

pub fn find_escrow_pda(locker: &Pubkey, vault: &Pubkey, locked_voter_program: &Pubkey) -> Pubkey {
    let (escrow, _) = Pubkey::find_program_address(
        &[b"Escrow".as_ref(), locker.as_ref(), vault.as_ref()],
        &locked_voter_program,
    );
    escrow
}

pub fn find_lst_escrow_ata_pda(unstaking: &Pubkey) -> Pubkey {
    let (escrow, _) = Pubkey::find_program_address(
        &[b"unstaking_escrow".as_ref(), unstaking.as_ref()],
        &neptune::ID,
    );
    escrow
}

pub fn find_lst_mint_pda(vault: &Pubkey, neptune_program: &Pubkey) -> Pubkey {
    let (mint, _) =
        Pubkey::find_program_address(&[b"lst".as_ref(), vault.as_ref()], &neptune_program);

    mint
}
