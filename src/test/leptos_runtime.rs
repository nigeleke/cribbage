use leptos::*;
use leptos_meta::provide_meta_context;

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
        let runtime = create_runtime();
        provide_meta_context();
        let view = (self.given)().into_view();
        (self.when)(&view);
        let rendered = view.render_to_string().to_string();
        (self.then)(rendered);
        runtime.dispose();
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
