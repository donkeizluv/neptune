// //! Macros

#[macro_export]
macro_rules! vault_seeds {
    ($vault:expr, $locker_pk:ident, $vault_owner:ident) => {{
        &[&[
            Vault::VAULT_SEED,
            $locker_pk.as_ref(),
            $vault_owner.as_ref(),
            &[$vault.bump],
        ]]
    }};
}

#[macro_export]
macro_rules! unwrap_ops {
    ($ops:expr) => {{
        $ops.ok_or(NeptuneError::ArithmeticOverflow)?
    }};

    ($ops:expr, $error:expr) => {{
        $ops.ok_or($error)?
    }};
}
