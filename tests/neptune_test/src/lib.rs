#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod stake_test {

    use std::rc::Rc;

    use anchor_client::{
        solana_sdk::{
            account::Account, commitment_config::CommitmentConfig, message::Message, pubkey,
            signature::Keypair, signer::Signer, transaction::Transaction,
        },
        Client, Cluster,
    };
    use anchor_lang::{declare_program, prelude::Pubkey, system_program};
    use anchor_spl::{associated_token, token};
    use litesvm::LiteSVM;
    use neptune::client::{accounts::CreateVault, args::CreateVault as CreateVaultArgs};

    declare_program!(neptune);

    #[test]
    fn test_stake() {
        let player_kp = Keypair::new();

        let mut svm = LiteSVM::new().with_blockhash_check(false);

        svm.airdrop(&player_kp.pubkey(), 10_000_000_000).unwrap();

        // load neptune program
        let neptune_program_id = neptune::ID;
        let neptune_bin = include_bytes!("../../../target/deploy/neptune.so");
        svm.add_program(neptune_program_id, neptune_bin);

        // load locked_voter program
        let locked_voter_program_id = pubkey!("voTpe3tHQ7AjQHMapgSue2HJFAh2cGsdokqN3XqmVSj");
        let locked_voter_bin = include_bytes!("../../../program_bytes/locked_voter.so");
        svm.add_program(locked_voter_program_id, locked_voter_bin);

        // load jup mint
        let jup_mint_pk = pubkey!("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN");
        let jup_mint_account =
            include_bytes!("../../../program_bytes/JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN");
        svm.set_account(
            jup_mint_pk,
            Account {
                lamports: 1_000_000_000,
                data: jup_mint_account.to_vec(),
                owner: token::ID,
                executable: false,
                rent_epoch: 0,
            },
        )
        .unwrap();
        // load locker
        let locker_pk = pubkey!("CVMdMd79no569tjc5Sq7kzz8isbfCcFyBS5TLGsrZ5dN");
        let locker_account_bytes =
            include_bytes!("../../../program_bytes/CVMdMd79no569tjc5Sq7kzz8isbfCcFyBS5TLGsrZ5dN");
        svm.set_account(
            locker_pk,
            Account {
                lamports: 1_000_000_000,
                data: locker_account_bytes.to_vec(),
                owner: locked_voter_program_id,
                executable: false,
                rent_epoch: 0,
            },
        )
        .unwrap();

        // Create program client
        let provider = Client::new_with_options(
            Cluster::Localnet,
            Rc::new(&player_kp),
            CommitmentConfig::confirmed(),
        );
        let program = provider.program(neptune::ID).unwrap();

        // prep accounts

        let (vault_pda, _) = Pubkey::find_program_address(
            &[
                b"vault".as_ref(),
                locker_pk.as_ref(),
                player_kp.pubkey().as_ref(),
            ],
            &neptune::ID,
        );
        let (escrow_pda, _) = Pubkey::find_program_address(
            &[b"Escrow".as_ref(), locker_pk.as_ref(), vault_pda.as_ref()],
            &locked_voter_program_id,
        );

        let (lst_mint_pk, _) =
            Pubkey::find_program_address(&[b"lst".as_ref(), vault_pda.as_ref()], &neptune::ID);

        let create_vault_ix = program
            .request()
            .accounts(CreateVault {
                signer: player_kp.pubkey(),
                locker: locker_pk,
                escrow: escrow_pda,
                vault: vault_pda,
                lst_mint: lst_mint_pk,
                utoken_mint: jup_mint_pk,
                vault_owner: player_kp.pubkey(),

                locked_voter_program: locked_voter_program_id,
                associated_token_program: associated_token::ID,
                token_program: token::ID,
                system_program: system_program::ID,
            })
            .args(CreateVaultArgs { fees_bps: 1000 })
            .instructions()
            .unwrap();

        let create_vault_tx = Transaction::new(
            &[&player_kp],
            Message::new(&create_vault_ix, Some(&player_kp.pubkey())),
            svm.latest_blockhash(),
        );

        // svm.send_transaction(create_vault_tx).unwrap();

        assert!(
            svm.send_transaction(create_vault_tx).is_ok(),
            "tx should success"
        );

        let vault_account = svm.get_account(&vault_pda).unwrap();
        assert_eq!(
            vault_account.owner,
            neptune::ID,
            "vault account should be inititalized"
        );
    }
}
