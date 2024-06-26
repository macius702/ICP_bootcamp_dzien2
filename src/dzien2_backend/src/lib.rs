use std::cell::RefCell;

thread_local! {
    static TEXTS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
pub fn add_text(text: String) {
    TEXTS.with(|texts :&RefCell<Vec<String>>| {
        texts.borrow_mut().push(text);
    });
}

//delete text
#[ic_cdk::update]
pub fn delete_text(index: usize) {
    TEXTS.with(|texts| {
        texts.borrow_mut().remove(index);
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


#[ic_cdk::update]
fn edit_text(index: usize, nowy_wpis: String) {
    TEXTS.with(|texts | {
        let mut binding = texts.borrow_mut();
        let wpis  = binding.get_mut(index);

        let stary_wpis   = wpis.unwrap();
        *stary_wpis = nowy_wpis;

        
    });
}




