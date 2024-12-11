// use magika::{ContentType, FeaturesOrRuled, InferredType, RuledType};
// use sqlx::FromRow;
// use syntect::{
//     html::{ClassStyle, ClassedHTMLGenerator},
//     parsing::{Scope, SyntaxSet},
//     util::LinesWithEndings,
// };

// use crate::db;

// #[derive(Debug)]
// pub struct Snippet {
//     content: String,
//     html: String,
// }

// pub struct Generator<'a> {
//     pub detector: &'a magika::Session,
//     pub syntax_set: &'a SyntaxSet,
//     pub pool: &'a db::Pool,
// }

// pub struct Filter {
//     pub cursor: Option<i32>,
//     pub search: String,
// }

// pub fn predict_scope(detector: &magika::Session, content: &str) -> Option<Scope> {
//     let pred = detector.identify_content_sync(content.as_bytes()).unwrap();
//     println!("{:?}", pred);

//     match pred {
//         magika::FileType::Inferred(inference) => {
//             let InferredType {
//                 content_type,
//                 score,
//             } = inference;

//             println!("{content} = {score}, {:?}", content_type);

//             return content_type_to_scope(if score < 0.5 {
//                 ContentType::Txt
//             } else {
//                 content_type
//             });
//         }
//         magika::FileType::Ruled(ruled) => {
//             let RuledType { content_type, overruled } = ruled;
//             println!("{:?}", overruled);
//             return content_type_to_scope(content_type);
//         }
//         _ => panic!(),
//     }
// }

// impl Generator<'_> {
//     pub async fn list_snippets(&self, filter: Filter) -> Vec<Snippet> {
//         #[derive(FromRow)]
//         struct Row {
//             content: String,
//         }

//         let rows: Vec<Row> = sqlx::query_as::<_, Row>(
//             "
//                 SELECT pb_item.id, pb_item.content
//                 FROM pb_item
//                 JOIN pb_item_fts ON pb_item.id = pb_item_fts.rowid
//                 WHERE pb_item_fts.rowid > ?
//                 LIMIT 50
//             ",
//         )
//         .bind(filter.cursor.unwrap_or(0))
//         .bind(&filter.search)
//         .fetch_all(self.pool)
//         .await
//         .unwrap();

//         rows.into_iter()
//             .map(|row| {
//                 let Row { content } = row;
//                 let scope = predict_scope(self.detector, &content);
//                 let html = match scope {
//                     Some(scope) => {
//                         let sr = self.syntax_set.find_syntax_by_scope(scope).unwrap();
//                         let mut generator = ClassedHTMLGenerator::new_with_class_style(
//                             sr,
//                             self.syntax_set,
//                             ClassStyle::Spaced,
//                         );

//                         for line in LinesWithEndings::from(&content) {
//                             generator
//                                 .parse_html_for_line_which_includes_newline(line)
//                                 .unwrap();
//                         }
//                         let inner_html = generator.finalize();

//                         format!("<pre class=\"code\">{inner_html}</pre>")
//                     }
//                     None => content.clone(),
//                 };

//                 Snippet { content, html }
//             })
//             .collect()
//     }
// }

// pub fn content_type_to_scope(content_type: ContentType) -> Option<Scope> {
//     let str = match content_type {
//         ContentType::Asp => Some("source.asp"),
//         ContentType::Html => Some("text.html.basic"),
//         ContentType::Batch => Some("source.dosbatch"),
//         ContentType::Csproj => Some("source.nant-build"),
//         ContentType::Cs => Some("source.cs"),
//         ContentType::Cpp => Some("source.c++"),
//         ContentType::C => Some("source.c"),
//         ContentType::Css => Some("source.css"),
//         ContentType::Clojure => Some("source.clojure"),
//         ContentType::Diff => Some("source.diff"),
//         ContentType::Erlang => Some("source.erlang"),
//         ContentType::Go => Some("source.go"),
//         ContentType::Groovy => Some("source.groovy"),
//         ContentType::Haskell => Some("source.haskell"),
//         ContentType::Latex => Some("text.tex.latex"),
//         ContentType::Java => Some("source.java"),
//         ContentType::Json => Some("source.json"),
//         ContentType::Javascript => Some("source.js"),
//         ContentType::Bib => Some("text.bibtex"),
//         ContentType::Lisp => Some("source.lisp"),
//         ContentType::Lua => Some("source.lua"),
//         ContentType::Makefile => Some("source.makefile"),
//         ContentType::Markdown => Some("text.html.markdown"),
//         ContentType::Matlab => Some("source.matlab"),
//         ContentType::Ocaml => Some("source.ocaml"),
//         ContentType::Objectivec => Some("source.objc"),
//         ContentType::Php => Some("source.php"),
//         ContentType::Pascal => Some("source.pascal"),
//         ContentType::Perl => Some("source.perl"),
//         ContentType::Python => Some("source.python"),
//         ContentType::R => Some("source.r"),
//         ContentType::Ruby => Some("source.ruby"),
//         ContentType::Rst => Some("text.restructuredtext"),
//         ContentType::Rust => Some("source.rust"),
//         ContentType::Sql => Some("source.sql"),
//         ContentType::Scala => Some("source.scala"),
//         ContentType::Shell => Some("source.shell.bash"),
//         ContentType::Tcl => Some("source.tcl"),
//         ContentType::Xml => Some("text.xml"),
//         ContentType::Yaml => Some("source.yaml"),
//         _ => Some("text.plain"),
//     };

//     str.map(|x| Scope::new(x).unwrap())
// }
