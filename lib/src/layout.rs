extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Layout {
    header: LayoutItem,
    items: Vec<LayoutItem>,
}

#[wasm_bindgen]
pub struct LayoutItem {
    name: String,
    value: f64,
}

impl Layout {
    pub fn new(header: LayoutItem, items: Vec<LayoutItem>) -> Layout {
        return Layout{
            header: header,
            items: items,
        }
    }

    pub fn header(&self) -> &LayoutItem {
        return &self.header;
    }

    pub fn items(&self) -> &Vec<LayoutItem> {
        return &self.items;
    }
}

impl LayoutItem {
    pub fn new(name: String, value: f64) -> LayoutItem {
        return LayoutItem{
            name: name,
            value: value,
        }
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn value(&self) -> &f64 {
        return &self.value;
    }
}
