// native app entry_point

mod sketch;
use sketch::{run_app, Model};
use async_std::task::block_on;

fn main() {
	let model = Model {};
	block_on(async {
		run_app(model).await;
	});
}
