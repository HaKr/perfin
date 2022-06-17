use std::fs;

use handlebars::{handlebars_helper, Handlebars, JsonValue, RenderError, TemplateError};
use serde::Serialize;
use tracing::{debug, error};

pub struct HtmlTemplateRenderer {
    handlebars: Handlebars<'static>,
}

handlebars_helper!( add_one: |index: u64| index + 1 );

handlebars_helper!(upper: |name: String| format!("{}", first_letter_to_uppper_case(name.to_string())));

handlebars_helper!(plural: |count: u8, noun: String| {
    if count == 1 {
        noun
    } else {
        format!("{}s", noun)
    }
});

handlebars_helper!(named: |type_name: String| {
    let result = format!("script_node_{}", type_name);
    result
});

handlebars_helper!(is_eq: |val1: Option<String>, val2: Option<String> | {
    val1.eq(&val2)
});

handlebars_helper!(to_json: |value: JsonValue | {
    debug!("to_json called with {:?}", value);

    "See console"
});

handlebars_helper!(selected_if: |current: String, selected_code: Option<String> | {
    let mut result = "";
    if let Some(selected_code) = selected_code {
        if current.eq(selected_code.as_str()) {
            result = "selected"
        }
    };

    result
});

impl HtmlTemplateRenderer {
    pub fn new() -> Result<Self, TemplateError> {
        let mut result = Self {
            handlebars: Handlebars::new(),
        };

        result.register()?;

        return Ok(result);
    }

    fn register(&mut self) -> Result<(), TemplateError> {
        self.handlebars.register_helper("upper", Box::new(upper));
        self.handlebars.register_helper("plural", Box::new(plural));
        self.handlebars.register_helper("named", Box::new(named));
        self.handlebars.register_helper("is_eq", Box::new(is_eq));
        self.handlebars
            .register_helper("add_one", Box::new(add_one));
        self.handlebars
            .register_helper("to_json", Box::new(to_json));
        self.handlebars
            .register_helper("selected_if", Box::new(selected_if));

        register_files_from("./templates", &mut |partial_name, partial_file| {
            debug!("Template '{}' -> {}", partial_name, partial_file);
            self.handlebars
                .register_template_file(partial_name, partial_file)
        })?;

        return Ok(());
    }

    pub fn refresh_templates(&mut self) -> Result<(), TemplateError> {
        {
            self.handlebars.clear_templates();
        }

        self.register()
    }

    pub fn render<T>(&mut self, name: &str, data: &T) -> Result<String, RenderError>
    where
        T: Serialize,
    {
        let result = self.handlebars.render(name, data);
        return result;
    }
}

fn first_letter_to_uppper_case(s1: String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn register_files_from<A>(folder: &str, action: &mut A) -> Result<(), TemplateError>
where
    A: FnMut(&str, &str) -> Result<(), TemplateError>,
{
    if let Ok(partials) = fs::read_dir(folder) {
        for partial in partials {
            if let Ok(partial) = partial {
                let partial_path = partial.path();
                let partial_file = partial_path.clone().into_os_string().into_string().unwrap();
                if partial_path.is_file() {
                    let partial_name = partial_path.file_stem().unwrap().to_str().unwrap();
                    let partial_file = partial_path.clone().into_os_string().into_string().unwrap();

                    (action)(partial_name, partial_file.as_str())?;
                } else {
                    register_files_from(partial_file.as_str(), action)?;
                }
            } else {
                error!("Invalid dir entry");
            }
        }
    } else {
        error!("No partials found in {}", folder);
    }
    Ok(())
}
