// //! Macros

#[macro_export]
macro_rules! partial_unstaking_seeds {
    ($unstaking:expr, $unstaking_key:ident) => {{
        &[&[
            Unstaking::PARTIAL_UNSTAKING_SEED,
            $unstaking_key.as_ref(),
            &[$unstaking.bump],
        ]]
    }};
}

#[macro_export]
macro_rules! vault_seeds {
    ($vault:expr, $escrow_key:ident) => {{
        &[&[Vault::VAULT_SEED, $escrow_key.as_ref(), &[$vault.bump]]]
    }};
}
