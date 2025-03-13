#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod stake_test {

    use std::rc::Rc;

    use anchor_client::{
        solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair, signer::Signer},
        Client, Cluster,
    };
    use anchor_lang::{declare_program, prelude::Pubkey, system_program};
    use anchor_spl::{associated_token, token};
    use litesvm::LiteSVM;
    use neptune::client::accounts::CreateVault;

    declare_program!(neptune);

    #[test]
    fn test_stake() {
        let signer_keypair = Keypair::new();
        let keypair = Keypair::new();

        let mut svm = LiteSVM::new();
        svm.airdrop(&keypair.pubkey(), 10_000).unwrap();

        // load locked_voter
        let locked_voter_program_id = Pubkey::new_unique();
        let locked_voter_bin = include_bytes!("../../../program_bytes/locked_voter.so");
        svm.add_program(locked_voter_program_id, locked_voter_bin);

        // Create program client
        let provider = Client::new_with_options(
            Cluster::Localnet,
            Rc::new(&signer_keypair),
            CommitmentConfig::confirmed(),
        );
        let program = provider.program(neptune::ID).unwrap();

        let create_vault_tx = program
            .request()
            .accounts(CreateVault {
                signer: keypair.pubkey(),
                vault: Pubkey::new_unique(),
                escrow: Pubkey::new_unique(),
                locker: Pubkey::new_unique(),
                lst_mint: Pubkey::new_unique(),
                vault_owner: Pubkey::new_unique(),
                utoken_mint: Pubkey::new_unique(),

                locked_voter_program: locked_voter_program_id,
                associated_token_program: associated_token::ID,
                token_program: token::ID,
                system_program: system_program::ID,
            })
            .signer(&signer_keypair)
            .transaction()
            .unwrap();

        let meta = svm.send_transaction(create_vault_tx).unwrap();
        println!("{}", meta.logs[0]);
    }
}
