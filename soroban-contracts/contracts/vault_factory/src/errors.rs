//! Contract error codes for VaultFactory.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    VaultAlreadyExists = 1,
    VaultNotFound      = 2,
    NotAuthorized      = 3,
}
