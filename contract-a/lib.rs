#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod contract_a {
    // use contract_b::{ContractB, ContractBRef};
    use ink_lang::{codegen::EmitEvent, reflect::ContractEventBase};

    #[ink(storage)]
    pub struct ContractA {}

    #[ink(event)]
    pub struct ContractACalled {}

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Default,
    }

    /// Result type
    pub type Result<T> = core::result::Result<T, Error>;
    /// Event type
    pub type Event = <ContractA as ContractEventBase>::Type;

    impl ContractA {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }

        #[ink(message)]
        pub fn call(&self) -> Result<()> {
            Self::emit_event(self.env(), Event::ContractACalled(ContractACalled {}));

            Ok(())
        }
    }
}
