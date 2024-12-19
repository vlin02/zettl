use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

use crate::{db, lookup};

pub struct Clipboard {
    pub ort: ort::session::Session,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub lookup: lookup::LookupTable,
    pub pool: db::Pool,
}

impl Clipboard {
    pub fn new(pool: db::Pool) -> Clipboard {
        Clipboard {
            ort: ort::session::Session::builder()
                .unwrap()
                .commit_from_memory(include_bytes!("data/model.onnx"))
                .unwrap(),
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            lookup: lookup::LookupTable::new(),
            pool,
        }
    }
}
