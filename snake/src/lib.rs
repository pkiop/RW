mod random;
mod snake;

use std::{cell::RefCell, rc::Rc};
use js_sys::Function;
use self::snake::SnakeGame;
use wasm_bindgen::{prelude::*, UnwrapThrowExt, JsCast};
use web_sys::{console, window};


#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting ....".into());
    let game = Rc::new(RefCell::new(SnakeGame::new(20, 20))); // Rc : reference counter
    let tick_closure = Closure::wrap(Box::new({
        let mut game = game.clone(); 
        move || game.borrow_mut().tick()}) as Box<dyn FnMut()>);

    window()
        .unwrap_throw()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(), 
        500,
    ).unwrap_throw();

}