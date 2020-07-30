extern crate wasm_bindgen;

use crate::layout::Layout;
use crate::layout::LayoutItem;
use regex::Regex;
use wasm_bindgen::prelude::*;

mod layout;

#[wasm_bindgen]
extern {
    fn register(s: &str);
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let result = render();

    match result {
        Ok(_) => return Ok(()),
        _ => {
            let window = web_sys::window().expect("no global `window` exists");

            let document = window.document().expect("should have a document on window");
            let container = document.get_element_by_id("container").expect("no container in app");

            container.set_inner_html("Não consegui montar a tela 😢 </br> Verifique se a url está preenchida com os dados do formulário!");

            return Ok(());
        }
    };
}


fn render() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");

    let document = window.document().expect("should have a document on window");
    let container = document.get_element_by_id("container").expect("no container in app");

    let layout = get_layout_from_query(&window)?;

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

fn get_layout_from_query(window: &web_sys::Window) -> Result<Layout, &str> {
    let search = window.location().search();

    let base64 = match search {
        Ok(v) => v.replace("?q=", ""),
        _ => return Err("não achei a querystring")
    };

    if base64.len() == 0 {
        return Err("sem dados do formulário");
    }

    let layout = build_layout(base64.as_str());

    return Ok(layout);
}

#[wasm_bindgen]
pub fn get_base_value() -> f64 {
    let window = web_sys::window().expect("no global `window` exists");
    let layout = get_layout_from_query(&window);

    let value: f64 = *layout.unwrap().header().value();

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
pub fn calculate(base: f64, values: Vec<f64>, income_text: &str) {
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
    result_span.set_attribute("style", "").unwrap();

    details_span.set_inner_html(&format!("({:.0}% de {:.0}%)", sum * 100f64, base * 100f64));
    details_span.set_attribute("style", "").unwrap();

    
    let numeric_pattern = Regex::new(r"[^\d\.]").unwrap();    
    let numeric_part = numeric_pattern.replace_all(income_text, "");
    
    if numeric_part.len() == 0 {
        return;
    }
    
    let income = numeric_part.parse::<f64>().unwrap();
    
    if income >= 0f64 {
        let amount = income * result;
        set_value(amount, "result-amount", &document);
    }
    
    let text_pattern = Regex::new(r"[^a-zA-Z]").unwrap();
    let text_part = text_pattern.replace_all(income_text, "");

    console_log(&text_part);

    set_text(&text_part, "extra", &document);
}

fn set_value(value: f64, element_id: &str, document: &web_sys::Document) {
    let element = document.get_element_by_id(element_id).unwrap();

    element.set_inner_html(&format!("{:.2}", value));
    element.set_attribute("style", "").unwrap();
}

fn set_text(text: &str, element_id: &str, document: &web_sys::Document) {
    let element = document.get_element_by_id(element_id).unwrap();

    element.set_inner_html(text);
}

fn build_layout(input_base64: &str) -> Layout {
    let values = base64::decode(input_base64).unwrap();

    let string_values = std::str::from_utf8(&values).unwrap();

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
