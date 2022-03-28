pub mod comp_link {
    use linkme::distributed_slice;
    #[distributed_slice]
    pub static COMPONENTS: [&'static str] = [..];
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
