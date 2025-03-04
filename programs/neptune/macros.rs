// //! Macros

// /// Generates the signer seeds for an [crate::Vault].
// #[macro_export]
// macro_rules! vault_seeds {
//     ($vault: expr) => {
//         &[&[
//             b"NeptuneVault" as &[u8],
//             &$vault.owner.as_ref(),
//             &$vault.mint.as_ref(),
//             &[$vault.bump],
//         ]]
//     };
// }
