#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        String::from(
            name.rsplit("::")
                .find(|&part| part != "f" && part != "{{closure}}")
                .expect("Short function name"),
        )
    }};
}
