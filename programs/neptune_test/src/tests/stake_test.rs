

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod stake_test {
    use litesvm::LiteSVM;
    use solana_message::Message;
    use solana_pubkey::Pubkey;
    use solana_system_interface::instruction::transfer;
    use solana_keypair::Keypair;
    use solana_signer::Signer;
    use solana_transaction::Transaction;
    use anchor_lang::{solana_program::stake::instruction};
    use neptune::instuctions::stake::Stake;

    #[test]
    fn test_stake() {

        let mut lite_svm = LiteSVM::new();
        let signer_keypair = Keypair::new();
        // Create a new Keypair
        let keypair = Keypair::new();

        // Get the public key from the Keypair
        let pubkey = keypair.pubkey();

        // Use the Keypair as a Signer
        let signer: &dyn Signer = &keypair;

        let vault = Pubkey::new_unique();
        let mut svm = LiteSVM::new();
        svm.airdrop(&signer.pubkey(), 10_000).unwrap();
  
        let stake = Stake {
            signer: signer.to_account_info(),
        };
    }
}