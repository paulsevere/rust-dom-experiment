#![allow(warnings)]
#![feature(link_args)]
#![feature(link_args)]
#[link_args = "-s EXPORTED_FUNCTIONS=['_exec']"]
#[macro_use]
extern crate dump;

#[macro_use]
extern crate stdweb;
extern crate euclid;
extern crate cgmath;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod sim;


use sim::*;

use stdweb::web::{document, IEventTarget, Element, NodeList, Node, INode, IElement};

use stdweb::web::event::{IEvent, ClickEvent};
use stdweb::web::html_element::InputElement;
use stdweb::event_loop;
use stdweb::unstable::TryInto;
use std::ops::DerefMut;

// mod balls;

// use balls::*;
extern "C" {}

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}


fn console(outstr: String) {
    println!("{}", outstr);
}


fn get_links() -> NodeList {
    document().query_selector_all("a")
}

fn buttonClickHandler() {
    let button: Element = document().query_selector("#button").unwrap();
    dump!(button);
    // button.try_into().unwrap();
    button.add_event_listener(enclose!( (button) move|_:ClickEvent|{
        button.class_list().add("clicked")
    }));
}
#[no_mangle]
pub fn exec() {
    stdweb::initialize();
    let mut ps = points();

    js!{
        cool(@{ps});
    }

    // run(&mut ps);

    // stdweb::event_loop();


}

fn main() {}


// let links = get_links();
// links
//     .iter()
//     .map(|link| {
//         let elem: Element = link.try_into().unwrap();
//         elem.class_list().add("sweet");

//     })
//     .count();
// buttonClickHandler();