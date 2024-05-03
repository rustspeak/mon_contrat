#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]

mod mon_contrat {
    //Defines the storage of your contract.
    // Add new fields to the below struct in order
    // to add new static storage fields to your contract

    #[ink(storage)]
    pub struct Incrementer {
        value : i32,
    }
    impl  Incrementer {

        #[ink(constructor)]
        pub fn  new(init_value : i32) -> Self {
            Self {value : init_value }
        } 

        #[ink(constructor)]
        pub fn default()  -> Self {
            Self {
                value : 0,
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[cfg(test)]
        fn  default_works(){
            let  contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink(message)]
        pub fn modifie(&mut self, by : i32){
            self.value += by;
        }

        #[cfg(test)]
        fn it_works(){
            let mut contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
            contract.modifie(5);
            assert_eq!(contract.get(), 5);
            contract.modifie(10);
            assert_eq!(contract.get(), 15);
            contract.modifie(-12);
            assert_eq!(contract.get(), 3);
        }
    }
    

   /*  impl MonContrat {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let mon_contrat = MonContrat::default();
            assert_eq!(mon_contrat.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut mon_contrat = MonContrat::new(false);
            assert_eq!(mon_contrat.get(), false);
            mon_contrat.flip();
            assert_eq!(mon_contrat.get(), true);
        }
    }*/
}
