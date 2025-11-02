use leptos::mount::mount_to_body;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Portfolio Tracker"</h1>
            <p>"Compare stocks, Bitcoin, and gold performance"</p>
        </main>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
