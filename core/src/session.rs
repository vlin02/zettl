use syntect::parsing::SyntaxSet;

use crate::{db, detection::LookupTable};

pub struct Session<'a> {
  pub lookup: LookupTable,
  pub syntax_set: SyntaxSet,
  pub pool: &'a db::Pool,
  pub ort: ort::session::Session
}
