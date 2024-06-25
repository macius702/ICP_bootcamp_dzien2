use std::cell::RefCell;

thread_local! {
    static TEXTS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
pub fn add_text(text: String) {
    TEXTS.with(|texts| {
        texts.borrow_mut().push(text);
    });
}

#[ic_cdk::query]
pub fn get_all_texts() -> Vec<String> {
    TEXTS.with(|texts| {
        texts.borrow().clone()
    })
}

#[ic_cdk::query]
fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}





