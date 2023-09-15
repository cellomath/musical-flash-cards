// build with "wasm-pack build --target web"

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, HtmlInputElement, HtmlSelectElement, SvgsvgElement};

mod cards;
use cards::{status_card, Card, CelloCardGenerator, Note};

const SVG_NAMESPACE: Option<&'static str> = Some("http://www.w3.org/2000/svg");

fn create_controls() {
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document
        .get_elements_by_class_name("navigation")
        .get_with_index(0)
        .unwrap();
    let button_svg: SvgsvgElement = document
        .create_element_ns(SVG_NAMESPACE, "svg")
        .unwrap()
        .dyn_into()
        .unwrap();
    button_svg.set_attribute("viewBox", "0 0 100 25").unwrap();
    button_svg.set_attribute("width", "100%").unwrap();
    button_svg.set_attribute("height", "100%").unwrap();

    let arrow = document.create_element_ns(SVG_NAMESPACE, "path").unwrap();
    arrow.set_id("right_arrow");
    arrow
        .set_attribute("d", "m 75,12 h 13 v -10 l 10,10 -10,10 v -10 z")
        .unwrap();
    arrow
        .set_attribute(
            "style",
            "fill: #000000; fill-opacity: 1; stroke: black; stroke-width: 1;",
        )
        .unwrap();
    button_svg.append_child(&arrow).unwrap();

    let arrow = document.create_element_ns(SVG_NAMESPACE, "path").unwrap();
    arrow.set_id("left_arrow");
    arrow
        .set_attribute("d", "m 25,12 h -13 v -10 l -10,10 10,10 v -10 z")
        .unwrap();
    arrow
        .set_attribute(
            "style",
            "fill: #000000; fill-opacity: 1; stroke: black; stroke-width: 1;",
        )
        .unwrap();
    button_svg.append_child(&arrow).unwrap();

    let text = document.create_element_ns(SVG_NAMESPACE, "text").unwrap();
    text.set_text_content(Some("Navigate"));
    for (name, val) in [
        ("x", "50"),
        ("y", "15"),
        ("font-size", "10"),
        ("text-anchor", "middle"),
        ("fill", "black"),
        ("id", "navigation_text"),
    ] {
        text.set_attribute(name, val).unwrap();
    }
    button_svg.append_child(&text).unwrap();
    div.replace_children_with_node_1(&button_svg);
}

macro_rules! update_x_notes_fn {
    ($fn_name:ident,$name:literal) => {
        #[wasm_bindgen]
        pub fn $fn_name() {
            let document = web_sys::window().unwrap().document().unwrap();
            let display = document
                .get_element_by_id(concat!($name, "_range_notes"))
                .unwrap();
            let min = document
                .get_element_by_id(concat!($name, "_clef_min"))
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
            let max = document
                .get_element_by_id(concat!($name, "_clef_max"))
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
            let min = min
                .parse::<u8>()
                .map(|m| Note::from_midi(m).to_string())
                .or(min.parse::<Note>().map(|n| n.to_string()));
            let max = max
                .parse::<u8>()
                .map(|m| Note::from_midi(m).to_string())
                .or(max.parse::<Note>().map(|n| n.to_string()));
            let min = min.as_deref().unwrap_or("Not a note");
            let max = max.as_deref().unwrap_or("Not a note");

            display.set_inner_html(&format!("{} &#8594; {}", min, max));
        }
    };
}
update_x_notes_fn!(update_bass_notes, "bass");
update_x_notes_fn!(update_treble_notes, "treble");
update_x_notes_fn!(update_tenor_notes, "tenor");
update_x_notes_fn!(update_alto_notes, "alto");
#[wasm_bindgen]
pub fn load_preset() {
    let document = web_sys::window().unwrap().document().unwrap();
    let preset = document.get_element_by_id("presets").unwrap().dyn_into::<HtmlSelectElement>().unwrap().value();
    match preset.as_str() {
        "no_sharps_flats" => CelloCardGenerator::no_sharps_flats().write_settings(),
        "one_flat" => CelloCardGenerator::one_flat().write_settings(),
        "one_sharp" => CelloCardGenerator::one_sharp().write_settings(),
        "two_flats" => CelloCardGenerator::two_flats().write_settings(),
        "two_sharps" => CelloCardGenerator::two_sharps().write_settings(),
        "three_flats" => CelloCardGenerator::three_flats().write_settings(),
        "three_sharps" => CelloCardGenerator::three_sharps().write_settings(),
        "tenor_clef_initial" => CelloCardGenerator::tenor_clef_initial().write_settings(),
        "tenor_clef_advanced" => CelloCardGenerator::tenor_clef_advanced().write_settings(),
        "treble_clef_initial" => CelloCardGenerator::treble_clef_initial().write_settings(),
        "treble_clef_advanced" => CelloCardGenerator::treble_clef_advanced().write_settings(),
        "advanced" => CelloCardGenerator::advanced().write_settings(),
        "impossible" => CelloCardGenerator::impossible().write_settings(),
        _ => {return;}
    }
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();

    use rand::thread_rng;
    let rng = Rc::new(RefCell::new(thread_rng()));

    let current_settings = Rc::new(RefCell::new(CelloCardGenerator::no_sharps_flats()));


    let cards = current_settings.borrow().card_generator(&mut *rng.borrow_mut());
    let cards: Rc<RefCell<Vec<Card>>> = Rc::new(RefCell::new(cards));
    let card_index = Rc::new(Cell::new(0usize));

    {
        let cards = cards.borrow();
        if let Some(card) = cards.get(card_index.get()) {
            let card: Element = card.into();
            card.set_id("card");
            //THIS IS A HACK:
            card.set_inner_html(&card.inner_html());
            let div = document
                .get_elements_by_class_name("main")
                .get_with_index(0)
                .unwrap();
            div.replace_children_with_node_1(&card);
        }
    }

    create_controls();
    document
        .get_element_by_id("navigation_text")
        .unwrap()
        .set_inner_html(&format!(
            "{}/{}",
            card_index.get() + 1,
            cards.borrow().len()
        ));

    let right_arrow = document.get_element_by_id("right_arrow").unwrap();
    let right_pressed = Rc::new(Cell::new(false));
    {
        let right_pressed = Rc::clone(&right_pressed);
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            right_pressed.set(true);
        });
        right_arrow
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let cards = Rc::clone(&cards);
        let card_index = Rc::clone(&card_index);
        let right_pressed = Rc::clone(&right_pressed);
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            let cards = cards.borrow();
            if right_pressed.get() {
                let new_index = (card_index.get()+1).min(cards.len() - 1);
                card_index.set(new_index);
                if let Some(card) = cards.get(new_index){
                    let card: Element = card.into();
                    card.set_id("card");
                    card.set_inner_html(&card.inner_html());
                    let document = web_sys::window().unwrap().document().unwrap();
                    let div = document
                        .get_elements_by_class_name("main")
                        .get_with_index(0)
                        .unwrap();
                    document
                        .get_element_by_id("navigation_text")
                        .unwrap()
                        .set_inner_html(&format!("{}/{}", card_index.get() + 1, cards.len()));
                    div.replace_children_with_node_1(&card);
                }
                right_pressed.set(false);
            }
        });
        right_arrow
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    let left_arrow = document.get_element_by_id("left_arrow").unwrap();
    let left_pressed = Rc::new(Cell::new(false));
    {
        let left_pressed = Rc::clone(&left_pressed);
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            left_pressed.set(true);
        });
        left_arrow
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let cards = Rc::clone(&cards);
        let card_index = Rc::clone(&card_index);
        let left_pressed = Rc::clone(&left_pressed);
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            let cards = cards.borrow();
            if left_pressed.get() {
                let new_index = card_index.get().checked_sub(1).unwrap_or(0);
                card_index.set(new_index);
                if let Some(card) = cards.get(card_index.get()) {
                    let card: Element = card.into();
                    card.set_id("card");
                    let document = web_sys::window().unwrap().document().unwrap();
                    let div = document
                        .get_elements_by_class_name("main")
                        .get_with_index(0)
                        .unwrap();
                    document
                        .get_element_by_id("navigation_text")
                        .unwrap()
                        .set_inner_html(&format!("{}/{}", card_index.get() + 1, cards.len()));
                    div.replace_children_with_node_1(&card);
                }
                left_pressed.set(false);
            }
        });
        left_arrow
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let cards = Rc::clone(&cards);
        let card_index = Rc::clone(&card_index);
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let cards = cards.borrow();
            match event.key().as_str(){
                "ArrowLeft" => {
                    let new_index = card_index.get().checked_sub(1).unwrap_or(0);
                    card_index.set(new_index);
                    if let Some(card) = cards.get(card_index.get()) {
                        let card: Element = card.into();
                        card.set_id("card");
                        let document = web_sys::window().unwrap().document().unwrap();
                        let div = document
                            .get_elements_by_class_name("main")
                            .get_with_index(0)
                            .unwrap();
                        document
                            .get_element_by_id("navigation_text")
                            .unwrap()
                            .set_inner_html(&format!("{}/{}", card_index.get() + 1, cards.len()));
                        div.replace_children_with_node_1(&card);
                    }
                },
                "ArrowRight" => {
                    let new_index = (card_index.get()+1).min(cards.len() - 1);
                    card_index.set(new_index);
                    if let Some(card) = cards.get(new_index){
                        let card: Element = card.into();
                        card.set_id("card");
                        card.set_inner_html(&card.inner_html());
                        let document = web_sys::window().unwrap().document().unwrap();
                        let div = document
                            .get_elements_by_class_name("main")
                            .get_with_index(0)
                            .unwrap();
                        document
                            .get_element_by_id("navigation_text")
                            .unwrap()
                            .set_inner_html(&format!("{}/{}", card_index.get() + 1, cards.len()));
                        div.replace_children_with_node_1(&card);
                    }
                },
                _ => {}
            }
        });
        web_sys::window().unwrap().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    let menu_icon = document.get_element_by_id("menu-icon").unwrap();

    let menu_toggled = Rc::new(Cell::new(false));
    let menu_pressed = Rc::new(Cell::new(false));
    {
        let menu_pressed = Rc::clone(&menu_pressed);
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            menu_pressed.set(true);
        });
        menu_icon
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let rng = Rc::clone(&rng);
        let current_settings = Rc::clone(&current_settings);

        let card_index = Rc::clone(&card_index);
        let cards = Rc::clone(&cards);
        let menu_toggled = Rc::clone(&menu_toggled);
        let menu_pressed = Rc::clone(&menu_pressed);
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            if menu_pressed.get() {
                menu_pressed.set(false);
                menu_toggled.set(!menu_toggled.get());

                let document = web_sys::window().unwrap().document().unwrap();
                let full_screen_menu = document
                    .get_element_by_id("full-screen-menu")
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .unwrap();
                if menu_toggled.get() {
                    for name in ["right", "bottom"] {
                        full_screen_menu.style().set_property(name, "0").unwrap();
                    }
                    current_settings.borrow().write_settings();
                } else {
                    log("updated settings");
                    for (name, value) in [("right", "100%"), ("bottom", "100%")] {
                        full_screen_menu.style().set_property(name, value).unwrap();
                    }
                    let new_settings = CelloCardGenerator::read_settings();
                    if &new_settings != &*current_settings.borrow() {
                        let new_cards = new_settings.card_generator(&mut *rng.borrow_mut());
                        let card: Element = new_cards.get(0).map(|c|c.into()).unwrap_or_else(|| status_card("Looks like there's no cards in this deck! <br> Try adjusting some options or using a preset."));
                        cards.replace(new_cards);
                        card_index.set(0);
                        card.set_id("card");
                        let document = web_sys::window().unwrap().document().unwrap();
                        let div = document
                            .get_elements_by_class_name("main")
                            .get_with_index(0)
                            .unwrap();
                        document
                            .get_element_by_id("navigation_text")
                            .unwrap()
                            .set_inner_html(&format!(
                                "{}/{}",
                                card_index.get() + 1,
                                cards.borrow().len()
                            ));
                        div.replace_children_with_node_1(&card);

                        current_settings.replace(new_settings);
                    }
                }
            }
        });
        menu_icon
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let closure = Closure::<dyn Fn()>::new(load_preset);
        let function = &closure.as_ref().unchecked_ref();
        document
            .get_element_by_id("presets")
            .unwrap()
            .add_event_listener_with_callback("change", function)
            .unwrap();
        closure.forget();
    };

    macro_rules! update_x_notes {
        ($fn_name:ident,$name:literal) => {
            let closure = Closure::<dyn Fn()>::new($fn_name);
            let function = &closure.as_ref().unchecked_ref();
            document
                .get_element_by_id(concat!($name, "_clef_min"))
                .unwrap()
                .add_event_listener_with_callback("change", function)
                .unwrap();
            document
                .get_element_by_id(concat!($name, "_clef_max"))
                .unwrap()
                .add_event_listener_with_callback("change", function)
                .unwrap();
            closure.forget();
        };
    }
    update_x_notes!(update_bass_notes, "bass");
    update_x_notes!(update_treble_notes, "treble");
    update_x_notes!(update_tenor_notes, "tenor");
    update_x_notes!(update_alto_notes, "alto");

    log("complete");
}
