#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]
#[cfg(test)]
mod tests {

    #[ink::test]
    fn default_works() {
        let factory = Factory::default();
        assert_eq!(factory.get(), false);
    }

    #[ink::test]
    fn it_works() {
        let mut factory = Factory::new(false);
        assert_eq!(factory.get(), false);
        factory.flip();
        assert_eq!(factory.get(), true);
    }
}
