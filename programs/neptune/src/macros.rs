// //! Macros

#[macro_export]
macro_rules! vault_seeds {
    ($vault:expr, $escrow_key:ident) => {{
        &[&[Vault::VAULT_SEED, $escrow_key.as_ref(), &[$vault.bump]]]
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
