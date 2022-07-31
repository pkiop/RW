mod random;
mod snake;

use std::{cell::RefCell, rc::Rc};
use js_sys::Function;
use self::snake::SnakeGame;
use wasm_bindgen::{prelude::*, UnwrapThrowExt, JsCast};
use web_sys::{console, window, Document, Element, HtmlElement, CssStyleDeclaration, HtmlDivElement};

thread_local! {
    static GAME: Rc<RefCell< SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(20, 20))); // Rc : reference counter
    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone()); 
        move || game.borrow_mut().tick()
    }) as Box<dyn FnMut()>);
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting ....".into());

    TICK_CLOSURE.with(|closure| {
        window()
        .unwrap_throw()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().dyn_ref::<Function>().unwrap_throw(), 
            500,
        )
        .unwrap_throw();
    });

    render();
}

pub fn render() {
    let document = window()
    .unwrap_throw()
    .document()
    .unwrap_throw();
    let root_container = 
        document
            .get_element_by_id("root")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();
    let width = GAME.with(|game| game.borrow().width); 
    let height = GAME.with(|game| game.borrow().height); 

    root_container.set_inner_html("");
    root_container.style().set_property("display", "grid").unwrap_throw();   
    root_container.style()
        .set_property(
            "grid-template", 
            &format!(
                "repeat({}, auto) / repeat({}, auto)", 
                height,width 
            )
        )
        .unwrap_throw();   
    
    for y in 0..height {

        for x in 0..width {
            let pos = (x, y);
            let field_element = document.create_element("div").unwrap_throw().dyn_into::<HtmlDivElement>().unwrap_throw();
            field_element.set_inner_text({
                if pos == GAME.with(|game| game.borrow().food) {
                    "🍎"
                } else if GAME.with(|game| game.borrow().snake.contains(&pos)) {
                    "🟩"
                } else {
                    "⬜️"
                }
            });
            root_container.append_child(&field_element).unwrap_throw();
        }
    }
        
}