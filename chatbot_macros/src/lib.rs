extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn module_configuration(items: TokenStream) -> TokenStream {
    let s = items.to_string();

    // Parse tokens into args
    let names_iter = s.split(",");
    let names: Vec<&str>  = names_iter.map(|name| name.trim()).collect();

    let quoted: Vec<String> = names.iter().map(|name| format!("\"{}\"", name)).collect();

    let module_const = format!(
        "const VALID_MODULES: [&'static str; {}] = [{}];",
        quoted.len(),
        quoted.join(", ")
    );

    let boilerplate_iter = names.iter().map(|name| {
        format!(
            "let {n}_status = match modules.get(\"{n}\") {{
                Some(status) => *status,
                _ => ModuleStatus::Disabled
            }};

            let {n}_button = Button::new(
                format!(\"{n}: {{}}\", {n}_status.to_string()),
                move |a| {{
                    {n}::config_view(a, {n}_status);
                }},
            );

            config_buttons.add_child({n}_button);",
            n = name
        )
    });

    let boilerplate: Vec<String> = boilerplate_iter.collect();

    let config_function = format!(
        "fn module_configuration(app: &mut Cursive) {{
            let modules = &app.user_data::<Data>().unwrap().modules;

            let mut config_buttons = LinearLayout::vertical();

            {}

            app.add_layer(
                Dialog::around(config_buttons)
                    .title(\"Bot configuration\")
                    .button(\"Save\", |a| {{
                        if let Err(why) = save_configuration(a) {{
                            panic!(\"Couldn't write to file: {{:?}}, why\");
                        }}
                        a.pop_layer();
                    }})
            );
        }}",
        boilerplate.join(" ")
    );

    let out = format!("{}\n{}", module_const, config_function);

    match out.parse() {
        Ok(stream) => stream,
        Err(why) => panic!("Syntax error in proc macro: {:?}", why)
    }
}
