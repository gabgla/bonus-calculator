extern crate wasm_bindgen;

use crate::layout::Layout;
use crate::layout::LayoutItem;
use wasm_bindgen::prelude::*;

mod layout;


#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");

    let document = window.document().expect("should have a document on window");
    let container = document.get_element_by_id("container").expect("no container in app");

    let layout = get_layout_from_query(&window);

    let header = document.create_element("div")?;
    header.set_id("card-header");

    let header_title = document.create_element("label")?;
    header_title.set_id("card-header-title");
    header_title.set_inner_html(layout.header().name());
    
    let header_value = document.create_element("label")?;
    header_value.set_id("card-header-value");
    header_value.set_class_name("value-display fancy-value");
    header_value.set_inner_html(&format!("{:.1}%", layout.header().value() * 100f64));

    header.append_child(&header_title)?;
    header.append_child(&header_value)?;
    
    container.append_child(&header)?;

    let mut count = 0;

    for item in layout.items() {
        count += 1;

        let block_id = format!("block_{}", count);
        let label_id = format!("label_{}", count);
        let input_id = format!("input_{}", count);
        let value_id = format!("value_{}", count);

        let block = document.create_element("div")?;
        block.set_id(&block_id);
        block.set_class_name("slidecontainer");

        let label = document.create_element("span")?;
        label.set_id(&label_id);
        label.set_attribute("for", &input_id)?;
        label.set_inner_html(item.name());
        label.set_class_name("slider-label");

        let max_value = (item.value() * 1000f64) as i32;

        let input = document.create_element("input")?;
        input.set_id(&input_id);
        input.set_attribute("type", "range")?;
        input.set_attribute("min", "0")?;
        input.set_attribute("max", &max_value.to_string())?;
        input.set_attribute("value", "0")?;
        input.set_class_name("slider");

        let value = document.create_element("label")?;
        value.set_id(&value_id);
        value.set_class_name("value-display");
        value.set_inner_html("0%");

        block.append_child(&label)?;
        block.append_child(&input)?;
        block.append_child(&value)?;

        container.append_child(&block)?;

        register(&input_id);
    }
    
    let results = match document.get_element_by_id("results") {
        None => return Ok(()),
        Some(x) => x
    };

    results.set_attribute("style", "")?;

    Ok(())
}

fn get_layout_from_query(window: &web_sys::Window) -> Layout {
    let search = window.location().search();

    let base64 = match search {
        Ok(v) => v.replace("?q=", ""),
        _ => panic!("can't parse search"),
    };

    let layout = build_layout(base64.as_str());

    return layout
}

#[wasm_bindgen]
extern {
    fn register(s: &str);
}

#[wasm_bindgen]
pub fn get_base_value() -> f64 {
    let window = web_sys::window().expect("no global `window` exists");
    let layout = get_layout_from_query(&window);

    let value: f64 = *layout.header().value();

    return value;
}

#[wasm_bindgen]
pub fn handle(input: &str, value: f64) {
    let group_id = match input.split("_").last() {
        None => return,
        Some(x) => x
    };
    
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let value_label_id = format!("value_{}", &group_id);
    let label = match document.get_element_by_id(&value_label_id) {
        None => return,
        Some(x) => x
    };

    label.set_inner_html(&format!("{:.1}%", value / 10f64));
}

#[wasm_bindgen]
pub fn calculate(base: f64, values: Vec<f64>, income: f64) {
    let sum: f64 = values.iter().map(|v| v / 1000f64).sum();

    let result = sum * base;

    let window = web_sys::window().expect("no global `window` exists");

    let document = window.document().expect("should have a document on window");
    let result_span = match document.get_element_by_id("result-value") {
        None => return,
        Some(x) => x
    };
    let details_span = match document.get_element_by_id("calc-details") {
        None => return,
        Some(x) => x
    };

    result_span.set_inner_html(&format!("{:.1}%", result * 100f64));
    match result_span.set_attribute("style", "") {
        Err(_) => return,
        Ok(x) => x
    };

    details_span.set_inner_html(&format!("({:.0}% de {:.0}%)", sum * 100f64, base * 100f64));
    match details_span.set_attribute("style", "") {
        Err(_) => return,
        Ok(x) => x
    };


    if income <= 0f64 {
        return;
    }

    let amount = income * result;

    let amount_span = match document.get_element_by_id("result-amount") {
        None => return,
        Some(x) => x
    };

    amount_span.set_inner_html(&format!("{:.2}", amount));
    match amount_span.set_attribute("style", "") {
        Err(_) => return,
        Ok(x) => x
    };
}

fn build_layout(input_base64: &str) -> Layout {
    let result = base64::decode(input_base64);

    let values = match result {
        Ok(values) => values,
        Err(e) => panic!(e),
    };

    let result = std::str::from_utf8(&values);

    let string_values = match result {
        Ok(values) => values,
        Err(e) => panic!(e),
    };

    let lines = string_values
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>();

    let header = build_item(lines[0].clone());
    let body = build_body(lines[1..].to_vec());

    let layout = Layout::new(header, body);

    return layout;
}

fn build_body(lines: Vec<&str>) -> Vec<LayoutItem> {
    return lines
        .iter()
        .map(|l| build_item(l))
        .collect::<Vec<LayoutItem>>();
}

fn build_item(line: &str) -> LayoutItem {
    let parts = line.split(",").map(String::from).collect::<Vec<String>>();

    let name = parts[0].clone();
    let value = parts[1].parse::<f64>().unwrap();

    let layout_item = LayoutItem::new(name, value);

    return layout_item;
}
