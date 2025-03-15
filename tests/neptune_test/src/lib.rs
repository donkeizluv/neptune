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
                accounts::{CreateVault, Stake, BeginUnstaking},
                args::{CreateVault as CreateVaultArgs, Stake as StakeArgs, BeginUnstaking as BeginUnstakingArgs},
            },
        },
        prep::{load_accounts, write_token_account},
    };
    use crate::locked_voter;
    use crate::pda::find_lst_escrow_ata_pda;
    use crate::prep::sync_clock;

    use anchor_client::{
        solana_sdk::{
            commitment_config::CommitmentConfig, message::Message, native_token::sol_to_lamports,
            signature::Keypair, signer::Signer, transaction::Transaction,
        },
        Client, Cluster,
    };
    use anchor_lang::{system_program, AccountDeserialize};
    use anchor_spl::{
        associated_token::{self, get_associated_token_address},
        token::{self, Mint, TokenAccount},
        token_2022::spl_token_2022::ui_amount_to_amount,
    };
    use spl_token::{
        solana_program::{program_option::COption, program_pack::Pack},
        state::{Account as SplTokenAccount, AccountState},
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

        svm.send_transaction(create_vault_tx).unwrap();

        let vault_info = svm.get_account(&vault_pk).unwrap();
        let vault_account =
            neptune::accounts::Vault::try_deserialize(&mut vault_info.data.as_slice()).unwrap();
        println!("vault_account {:?}", vault_account);    

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
            ui_amount_to_amount(500_000_f64, utoken_mint.decimals),
        )
        .unwrap();
        println!("player_utoken_ata {:?}", player_utoken_ata);  

        // Fetch the created token account
        let account_data = svm.get_account(&player_utoken_ata).unwrap();
        let token_account = SplTokenAccount::unpack(&account_data.data).unwrap();

         // Verify the account details
        assert_eq!(token_account.mint, jup_mint_pk, "Mint does not match");
        assert_eq!(token_account.owner, player_kp.pubkey(), "Owner does not match");
        assert_eq!(token_account.amount, ui_amount_to_amount(500_000_f64, utoken_mint.decimals), "Incorrect token amount");

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

        let vault_info = svm.get_account(&vault_pk).unwrap();
        let vault_account =
            neptune::accounts::Vault::try_deserialize(&mut vault_info.data.as_slice()).unwrap();
        println!("vault_account {:?}", vault_account);

        assert_eq!(
            vault_account.total_utoken_staked, 
            ui_amount_to_amount(stake_amt, utoken_mint.decimals),
            "total_utoken_staked should match stake amount"
        );

        // stake more 1000 utokens
        let stake_amt_2st = 2000_f64;
        let stake_ix_2st = neptune_program
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
                amount: ui_amount_to_amount(stake_amt_2st, utoken_mint.decimals),
            })
            .instructions()
            .unwrap();

        let stake_tx_2st = Transaction::new(
            &[&player_kp],
            Message::new(&stake_ix_2st, Some(&player_kp.pubkey())),
            svm.latest_blockhash(),
        );

        svm.send_transaction(stake_tx_2st).unwrap();

        let player_lst_ata_info = svm.get_account(&player_lst_ata).unwrap();
        let player_lst_ata_account =
            TokenAccount::try_deserialize(&mut player_lst_ata_info.data.as_slice()).unwrap();

        let total_amount = ui_amount_to_amount(stake_amt + stake_amt_2st, utoken_mint.decimals);    
        assert_eq!(
            player_lst_ata_account.amount,
            total_amount,
            "lst amount should match stake amount"
        );

        let vault_info = svm.get_account(&vault_pk).unwrap();
        let vault_account =
            neptune::accounts::Vault::try_deserialize(&mut vault_info.data.as_slice()).unwrap();

        assert_eq!(
            vault_account.total_utoken_staked, 
            total_amount,
            "total_utoken_staked should match stake amount"
        );

        let unstaking_amount = 1000_f64;

        let unstaking_kp = Keypair::new();
        let partial_unstaking_kp = Keypair::new();
        let lst_source_ata = get_associated_token_address(&player_kp.pubkey(), &lst_mint_pk,);
        let lst_escrow_pk = find_lst_escrow_ata_pda(&unstaking_kp.pubkey());

        let begin_unstake_ix = neptune_program
            .request()
            .accounts(BeginUnstaking {
                signer: player_kp.pubkey(),
                vault: vault_pk,
                locker: locker_pk,
                escrow: escrow_pk,
                lst_mint: lst_mint_pk,
                unstaking: unstaking_kp.pubkey(),
                partial_unstaking: partial_unstaking_kp.pubkey(),
                lst_source_ata: lst_source_ata,//
                lst_escrow_ata: lst_escrow_pk,//
                locked_voter_program: locked_voter_program_id,
                token_program: token::ID,
                system_program: system_program::ID,
                associated_token_program: associated_token::ID,
            })
            .args(BeginUnstakingArgs {
                amount: ui_amount_to_amount(unstaking_amount, utoken_mint.decimals),
            })
            .instructions()
            .unwrap();
            // let begin_unstake_tx = Transaction::new_signed_with_payer(
            //     &begin_unstake_ix,
            //     Some(&player_kp.pubkey()),
            //     &[&player_kp, &unstaking_kp, &partial_unstaking_kp],
            //     svm.latest_blockhash(),
            // );

            let mut begin_unstake_tx = Transaction::new_unsigned(Message::new(
                &begin_unstake_ix,
                Some(&player_kp.pubkey()),
            ));
            print!("account_keys 111111 {:?}", begin_unstake_tx.message.account_keys);

            begin_unstake_tx.sign(&[&player_kp, &unstaking_kp, &partial_unstaking_kp], svm.latest_blockhash());

            svm.send_transaction(begin_unstake_tx).unwrap();

    }
}
