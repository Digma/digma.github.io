use wasm_bindgen::prelude::wasm_bindgen;

mod sketch;
use sketch::{run_app, Model};
use async_std::task::block_on;

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	let model = Model {};
	block_on(async {
		run_app(model).await;
	});
}

// #[wasm_bindgen]
// pub fn update_message(selector: &str, message: &str) {
//     let window = web_sys::window().expect("Failed to load window");
//     let document = window.document().expect("Failed to load document");
//     let element = document.query_selector(selector).expect("Failed to load element");

//     if let Some(element) = element {
//         element.set_inner_html(message);
//     } else {
//         panic!("Failed to set inner html")
//     }
// }
