use yew_agent::PublicWorker;
fn main() {
    console_error_panic_hook::set_once();
    llama_wasm::Worker::register();
}
