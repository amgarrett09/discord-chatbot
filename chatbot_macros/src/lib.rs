extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn register_modules(items: TokenStream) -> TokenStream {
    let s = items.to_string();
    let names = s.split(" ");

    let quoted: Vec<String> = names.map(|name| format!("\"{}\"", name)).collect();

    // const VALID_MODULES: [&'static str; n] = [x, xs...];
    let module_const = format!(
        "const VALID_MODULES: [&'static str; {}] = [{}];",
        quoted.len(),
        quoted.join(", ")
    );

    module_const.parse().unwrap()
}
