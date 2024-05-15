pub use macros::form;

mod test {
    use macros::form;

    #[test]
    fn test() {
        form!(
            println!("Hello from test module");
        );

        println!("Hello from test module");
    }
}
