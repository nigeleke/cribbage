use leptos::*;

use std::panic;

pub type TestResult = Result<(), Box<dyn std::any::Any + Send>>;
type TestFn = fn() -> ();

pub struct LeptosRuntime;

impl LeptosRuntime {
    pub fn run(test_fn: TestFn) -> TestResult {
        let runtime = create_runtime();
        let result = panic::catch_unwind(test_fn);
        runtime.dispose();
        result
    } 
}
