use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::{self, SyntaxSet},
    util::LinesWithEndings,
};

use crate::detection::format::Format;

pub fn format_to_scope(format: Format) -> parsing::Scope {
    let str = match format {
        Format::Asp => "source.asp",
        Format::Html => "text.html.basic",
        Format::Batch => "source.dosbatch",
        Format::Cs => "source.cs",
        Format::C => "source.c",
        Format::Css => "source.css",
        Format::Go => "source.go",
        Format::Latex => "text.tex.latex",
        Format::Java => "source.java",
        Format::Json => "source.json",
        Format::JavaScript => "source.js",
        Format::Lisp => "source.lisp",
        Format::Makefile => "source.makefile",
        Format::Markdown => "text.html.markdown",
        Format::Php => "source.php",
        Format::Perl => "source.perl",
        Format::Python => "source.python",
        Format::Ruby => "source.ruby",
        Format::Rst => "text.restructuredtext",
        Format::Rust => "source.rust",
        Format::Sql => "source.sql",
        Format::Scala => "source.scala",
        Format::Shell => "source.shell.bash",
        Format::Xml => "text.xml",
        Format::Yaml => "source.yaml",
        _ => "text.plain",
    };

    parsing::Scope::new(str).unwrap()
}

pub fn generate_html(syntax_set: &SyntaxSet, content: &str, format: Format) -> String {
    let scope = format_to_scope(format);
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
