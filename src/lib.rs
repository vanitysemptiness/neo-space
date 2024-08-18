use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() {
    let mut state = State::new().await;

    loop {
        state.update();
        state.draw();
        next_frame().await
    }
}