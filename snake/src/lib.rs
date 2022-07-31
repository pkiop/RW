mod random;
mod snake;

use self::snake::{Direction, SnakeGame};

use js_sys::Function;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{
    console, window, CssStyleDeclaration, Document, Element, HtmlDivElement, HtmlElement,
    KeyboardEvent,
};

thread_local! {
    static GAME: Rc<RefCell< SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(20, 20))); // Rc : reference counter
    static HANDLE_TICK: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone());
        move || {
            game.borrow_mut().tick();
            render();
        }
    }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new({
        |evt: KeyboardEvent| {
            GAME.with(|game| {
                let direction = match &evt.key()[..] {
                "ArrowUp" => Some(Direction::Top),
                "ArrowDown" => Some(Direction::Bottom),
                "ArrowLeft" => Some(Direction::Left),
                "ArrowRight" => Some(Direction::Right),
                _ => None,
            };
            if let Some(direction) = direction {
                game.borrow_mut().change_direction(direction);
            }
        })
        }
    }) as Box<dyn FnMut(KeyboardEvent)>)
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting ....".into());

    HANDLE_TICK.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                100,
            )
            .unwrap_throw();
    });

    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });

    render();
}

pub fn render() {
    let document = window().unwrap_throw().document().unwrap_throw();
    let root_container = document
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();
    let width = GAME.with(|game| game.borrow().width);
    let height = GAME.with(|game| game.borrow().height);

    root_container.set_inner_html("");
    root_container
        .style()
        .set_property("display", "inline-grid")
        .unwrap_throw();
    root_container
        .style()
        .set_property(
            "grid-template",
            &format!("repeat({}, auto) / repeat({}, auto)", height, width),
        )
        .unwrap_throw();

    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);
            let field_element = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();
            field_element.set_inner_text({
                if GAME.with(|game| game.borrow().finished) {
                    "🐍"
                } else if pos == GAME.with(|game| game.borrow().food) {
                    "🍎"
                } else if GAME.with(|game| game.borrow().snake.get(0) == Some(&pos)) {
                    "🐲"
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
