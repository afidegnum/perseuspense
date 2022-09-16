mod templates;

use perseus::{Html, PerseusApp};

#[perseus::main(perseus_actix_web::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(crate::templates::index::get_template)
}
