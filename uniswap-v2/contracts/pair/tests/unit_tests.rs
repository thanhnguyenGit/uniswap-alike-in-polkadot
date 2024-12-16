#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[cfg(test)]
mod tests {
    use pair::pair::Pair;

    #[ink::test]
    fn default_works() {
        let pair = Pair::default();
        assert_eq!(pair.get(), false);
    }

    #[ink::test]
    fn it_works() {
        let mut pair = Pair::new(false);
        assert_eq!(pair.get(), false);
        pair.flip();
        assert_eq!(pair.get(), true);
    }
}
