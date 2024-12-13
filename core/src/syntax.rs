use syntect::{
    easy::HighlightLines,
    highlighting::Theme,
    html::append_highlighted_html_for_styled_line,
    parsing::{self, SyntaxReference, SyntaxSet},
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

pub fn highlight_lines(
    syntax_set: &SyntaxSet,
    syntax: &SyntaxReference,
    theme: &Theme,
    s: &str,
) -> Result<Vec<String>, syntect::Error> {
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut lines = Vec::new();

    for line in LinesWithEndings::from(s) {
        let regions = highlighter.highlight_line(line, syntax_set)?;

        let mut line = String::new();
        append_highlighted_html_for_styled_line(
            &regions[..],
            syntect::html::IncludeBackground::No,
            &mut line,
        )?;

        lines.push(line);
    }

    Ok(lines)
}
