use leptos::*;

pub struct LeptosRuntime<G, W, T, V>
where
    G: Fn() -> V,
    W: Fn(&View) -> (),
    T: Fn(String) -> (),
    V: IntoView {
    runtime: RuntimeId,
    given: G,
    when: W,
    then: T,
}

impl<G, W, T, V> LeptosRuntime<G, W, T, V>
where
    G: Fn() -> V,
    W: Fn(&View) -> (),
    T: Fn(String) -> (),
    V: IntoView {

    pub fn new(
        given: G,
        when: W,
        then: T) -> Self {
        Self { runtime: create_runtime(), given, when, then }
    }

    pub fn run(&self) {
        println!("a");
        let runtime = create_runtime();
        println!("b");
        let view = (self.given)().into_view();
        println!("c");
        (self.when)(&view);
        println!("d");
        let rendered = view.render_to_string().to_string();
        println!("e");
        (self.then)(rendered);
        println!("f");
        runtime.dispose();
        println!("g");
    }
}

impl<G, W, T, V> Drop for LeptosRuntime<G, W, T, V>
where
    G: Fn() -> V,
    W: Fn(&View) -> (),
    T: Fn(String) -> (),
    V: IntoView {

    fn drop(&mut self) {
        self.runtime.dispose()
    }
}
