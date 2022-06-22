#![cfg_attr(not(feature = "std"), no_std)]

pub use self::contract_b::{ContractB, ContractBRef};
use ink_lang as ink;

#[ink::contract]
mod contract_b {

    #[ink(storage)]
    pub struct ContractB {}

    #[ink(event)]
    pub struct ContractBCalled {
        #[ink(topic)]
        pub caller: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Default,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl ContractB {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message, selector = 1)]
        pub fn call(&self) -> Result<()> {
            let caller = self.env().caller();
            self.env().emit_event(ContractBCalled { caller });
            Ok(())
        }
    }
}
