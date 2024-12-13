use syntect::{
    easy::HighlightLines,
    highlighting::Theme,
    html::{append_highlighted_html_for_styled_line, ClassStyle, ClassedHTMLGenerator},
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

pub fn highlight_as_html(
    syntax_set: &SyntaxSet,
    syntax: &SyntaxReference,
    theme: &Theme,
    s: &str,
) -> Result<String, syntect::Error> {
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut output = String::new();
    for line in LinesWithEndings::from(s) {
        let regions = highlighter.highlight_line(line, syntax_set)?;
        append_highlighted_html_for_styled_line(
            &regions[..],
            syntect::html::IncludeBackground::No,
            &mut output,
        )?;
    }

    Ok(output)
}

pub fn preview_target_in_content(input: &str, target: &str, line_count: i32) -> String {
    let mut output = String::new();
    let mut curr_count = 0;

    for line in LinesWithEndings::from(input) {
        if curr_count > 0 || line.to_ascii_lowercase().contains(target) {
            output += line;
            curr_count += 1;
        }

        if curr_count == line_count {
            break;
        }
    }

    output
}
