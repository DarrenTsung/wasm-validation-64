#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(mut holder: TestTraitHolder) {
    let test_struct = TestStruct::new();
    let boxed = Box::new(test_struct);

    // If you comment this out, no errors from wasm-bindgen.
    holder.hold.push(boxed);
}

#[wasm_bindgen]
pub struct TestTraitHolder {
    hold: Vec<Box<TestTrait>>,
}

#[wasm_bindgen]
pub struct TestStruct {
    a: Rc<RefCell<u32>>,
    f: Rc<RefCell<ButtonState>>,
}

#[wasm_bindgen]
impl TestStruct {
    pub fn new() -> TestStruct {
        TestStruct {
            a: Rc::new(RefCell::new(0)),
            f: Rc::new(RefCell::new(ButtonState { hovered: true, clicked_in: false, clicked: true, })),
        }
    }
}

#[derive(Clone)]
struct ButtonState {
    pub hovered: bool,
    pub clicked_in: bool,
    // If you remove one of these fields, no errors from wasm-bindgen.
    pub clicked: bool,
}

pub trait TestTrait {
    fn hello(&self);
}

impl TestTrait for TestStruct {
    fn hello(&self) {
        // If you comment this out OR change `self.f` to `self.a`, no errors from wasm-bindgen
        let _hi = self.f.borrow().clone();
    }
}
