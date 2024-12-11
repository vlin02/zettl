use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::{self, SyntaxSet},
    util::LinesWithEndings,
};

use crate::detection::content_type::ContentType;

pub fn content_type_to_scope(content_type: ContentType) -> parsing::Scope {
    let str = match content_type {
        ContentType::Asp => "source.asp",
        ContentType::Html => "text.html.basic",
        ContentType::Batch => "source.dosbatch",
        ContentType::Cs => "source.cs",
        ContentType::C => "source.c",
        ContentType::Css => "source.css",
        ContentType::Go => "source.go",
        ContentType::Latex => "text.tex.latex",
        ContentType::Java => "source.java",
        ContentType::Json => "source.json",
        ContentType::JavaScript => "source.js",
        ContentType::Lisp => "source.lisp",
        ContentType::Makefile => "source.makefile",
        ContentType::Markdown => "text.html.markdown",
        ContentType::Php => "source.php",
        ContentType::Perl => "source.perl",
        ContentType::Python => "source.python",
        ContentType::Ruby => "source.ruby",
        ContentType::Rst => "text.restructuredtext",
        ContentType::Rust => "source.rust",
        ContentType::Sql => "source.sql",
        ContentType::Scala => "source.scala",
        ContentType::Shell => "source.shell.bash",
        ContentType::Xml => "text.xml",
        ContentType::Yaml => "source.yaml",
        _ => "text.plain",
    };

    parsing::Scope::new(str).unwrap()
}

pub fn generate_html(syntax_set: &SyntaxSet, content: &str, content_type: ContentType) -> String {
    let scope = content_type_to_scope(content_type);
    let syntax = syntax_set.find_syntax_by_scope(scope).unwrap();

    let mut generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, syntax_set, ClassStyle::Spaced);

    for line in LinesWithEndings::from(&content) {
        generator
            .parse_html_for_line_which_includes_newline(line)
            .unwrap();
    }

    let inner_html = generator.finalize();
    format!("<pre class=\"code\">{inner_html}</pre>")
}
