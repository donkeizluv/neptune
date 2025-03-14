use anchor_lang::declare_program;

mod pda;
mod prep;

declare_program!(neptune);
declare_program!(locked_voter);

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod neptune_test {
    use super::pda::{find_escrow_pda, find_lst_mint_pda, find_vault_pda};
    use super::{
        neptune::{
            self,
            client::{
                accounts::{CreateVault, Stake},
                args::{CreateVault as CreateVaultArgs, Stake as StakeArgs},
            },
        },
        prep::{load_accounts, write_token_account},
    };
    use crate::locked_voter;
    use crate::prep::sync_clock;

    use anchor_client::{
        solana_sdk::{
            commitment_config::CommitmentConfig, message::Message, native_token::sol_to_lamports,
            signature::Keypair, signer::Signer, transaction::Transaction,
        },
        Client, Cluster,
    };
    use anchor_lang::{system_program, AccountDeserialize};
    use anchor_spl::token::TokenAccount;
    use anchor_spl::{
        associated_token::{self, get_associated_token_address},
        token::{self, Mint},
        token_2022::spl_token_2022::ui_amount_to_amount,
    };

    use litesvm::LiteSVM;
    use std::rc::Rc;

    #[test]
    fn test_stake() {
        let vault_owner_kp = Keypair::new();
        let player_kp = Keypair::new();
        let fees = 1000_u16;

        let mut svm = LiteSVM::new().with_blockhash_check(false);
        svm.airdrop(&vault_owner_kp.pubkey(), sol_to_lamports(1000_f64))
            .unwrap();
        svm.airdrop(&player_kp.pubkey(), sol_to_lamports(1000_f64))
            .unwrap();
        let (locked_voter_program_id, _, locker_pk, jup_mint_pk) = load_accounts(&mut svm).unwrap();

        // Create program clients
        let provider = Client::new_with_options(
            Cluster::Mainnet,
            Rc::new(&vault_owner_kp),
            CommitmentConfig::processed(),
        );
        let neptune_program = provider.program(neptune::ID).unwrap();

        // sync svm clock with live cluster clock
        sync_clock(&mut svm, &neptune_program.rpc()).unwrap();

        // prep create vault accounts
        let vault_pk = find_vault_pda(&locker_pk, &vault_owner_kp.pubkey());
        let escrow_pk = find_escrow_pda(&locker_pk, &vault_pk, &locked_voter_program_id);
        let lst_mint_pk = find_lst_mint_pda(&vault_pk, &neptune::ID);

        let create_vault_ix = neptune_program
            .request()
            .accounts(CreateVault {
                signer: vault_owner_kp.pubkey(),
                locker: locker_pk,
                escrow: escrow_pk,
                vault: vault_pk,
                lst_mint: lst_mint_pk,
                utoken_mint: jup_mint_pk,
                vault_owner: vault_owner_kp.pubkey(),

                locked_voter_program: locked_voter_program_id,
                associated_token_program: associated_token::ID,
                token_program: token::ID,
                system_program: system_program::ID,
            })
            .args(CreateVaultArgs { fees_bps: fees })
            .instructions()
            .unwrap();

        let create_vault_tx = Transaction::new(
            &[&vault_owner_kp],
            Message::new(&create_vault_ix, Some(&vault_owner_kp.pubkey())),
            svm.latest_blockhash(),
        );

        // println!("{:?}", create_vault_tx.message.account_keys);

        svm.send_transaction(create_vault_tx).unwrap();

        let vault_info = svm.get_account(&vault_pk).unwrap();
        let vault_account =
            neptune::accounts::Vault::try_deserialize(&mut vault_info.data.as_slice()).unwrap();

        assert_eq!(vault_account.escrow, escrow_pk);
        assert_eq!(vault_account.fees_bps, fees);
        assert_eq!(vault_account.lst_mint, lst_mint_pk);
        assert_eq!(vault_account.owner, vault_owner_kp.pubkey());

        // read accounts
        let utoken_mint_info = svm.get_account(&jup_mint_pk).unwrap();
        let utoken_mint = Mint::try_deserialize(&mut utoken_mint_info.data.as_slice()).unwrap();

        let escrow_info = svm.get_account(&escrow_pk).unwrap();
        let escrow_account =
            locked_voter::accounts::Escrow::try_deserialize(&mut escrow_info.data.as_slice())
                .unwrap();

        let locker_info = svm.get_account(&locker_pk).unwrap();
        let locker_account =
            locked_voter::accounts::Locker::try_deserialize(&mut locker_info.data.as_slice())
                .unwrap();

        // mint player some utokens
        let player_utoken_ata = write_token_account(
            &mut svm,
            &player_kp.pubkey(),
            &jup_mint_pk,
            utoken_mint.decimals,
        )
        .unwrap();
        // stake
        let stake_amt = 1000_f64;
        // prep stake accounts
        let player_lst_ata = get_associated_token_address(&player_kp.pubkey(), &lst_mint_pk);
        let utoken_escrow_ata = get_associated_token_address(&escrow_pk, &jup_mint_pk);
        let stake_ix = neptune_program
            .request()
            .accounts(Stake {
                signer: player_kp.pubkey(),
                locker: locker_pk,
                escrow: escrow_pk,
                vault: vault_pk,
                lst_mint: lst_mint_pk,
                utoken_mint: jup_mint_pk,
                lst_ata: player_lst_ata,
                utoken_escrow_ata: utoken_escrow_ata,
                utoken_source_ata: player_utoken_ata,

                locked_voter_program: locked_voter_program_id,
                associated_token_program: associated_token::ID,
                token_program: token::ID,
                system_program: system_program::ID,
            })
            .args(StakeArgs {
                amount: ui_amount_to_amount(stake_amt, utoken_mint.decimals),
            })
            .instructions()
            .unwrap();

        let stake_tx = Transaction::new(
            &[&player_kp],
            Message::new(&stake_ix, Some(&player_kp.pubkey())),
            svm.latest_blockhash(),
        );

        svm.send_transaction(stake_tx).unwrap();

        let player_lst_ata_info = svm.get_account(&player_lst_ata).unwrap();
        let player_lst_ata_account =
            TokenAccount::try_deserialize(&mut player_lst_ata_info.data.as_slice()).unwrap();
        assert_eq!(
            player_lst_ata_account.amount,
            ui_amount_to_amount(stake_amt, utoken_mint.decimals),
            "lst amount should match stake amount"
        );
    }
}
