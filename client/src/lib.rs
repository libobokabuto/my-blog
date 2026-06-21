use app::App;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    _ = any_spawner::Executor::init_wasm_bindgen();
    leptos::mount::hydrate_body(App);
}
