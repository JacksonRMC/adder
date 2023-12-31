#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod adder {

    #[ink(storage)]
    pub struct Adder {
        counter: u32,
    }

    impl Adder {
        #[ink(constructor)]
        pub fn new() -> Self {
            todo!()
        }

        #[ink(message)]
        pub fn add(&mut self) {
            todo!()
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            todo!()
        }
    }
}
