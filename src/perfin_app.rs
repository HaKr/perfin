use std::sync::{Mutex, MutexGuard};

use crate::{html_template_renderer::HtmlTemplateRenderer, Ledger};

pub struct PerfinApp {
    template_renderer: Mutex<HtmlTemplateRenderer>,
    ledger: Mutex<Ledger>,
}

impl PerfinApp {
    pub fn new(template_renderer: HtmlTemplateRenderer, ledger: Ledger) -> Self {
        Self {
            template_renderer: Mutex::new(template_renderer),
            ledger: Mutex::new(ledger),
        }
    }

    pub fn use_template_renderer<'m>(&'m self) -> MutexGuard<'m, HtmlTemplateRenderer> {
        self.template_renderer.lock().expect("access to handlebars")
    }

    pub fn use_ledger<'m>(&'m self) -> MutexGuard<'m, Ledger> {
        self.ledger.lock().expect("access to Ledger")
    }
}
