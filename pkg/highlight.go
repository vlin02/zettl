package pkg

import (
	"bytes"
	"strings"

	chroma "github.com/alecthomas/chroma/v2"
	"github.com/alecthomas/chroma/v2/formatters/html"
	"github.com/alecthomas/chroma/v2/styles"
)

var auxStyle = getChromaStyle("onedark")

type noPreWrapper struct{}

func (noPreWrapper) Start(nl bool, style string) string { return "" }
func (noPreWrapper) End(nl bool) string                 { return "" }

func getChromaStyle(name string) *chroma.Style {
	if s := styles.Get(name); s != nil {
		return s
	}
	return styles.Get("onedark")
}

func ChromaCSSForStyle(styleName string) string {
	formatter := html.New(
		html.WithClasses(true),
		html.WithPreWrapper(noPreWrapper{}),
		html.WithLineNumbers(true),
	)

	var buf bytes.Buffer
	_ = formatter.WriteCSS(&buf, getChromaStyle(styleName))

	lines := strings.Split(buf.String(), "\n")
	filtered := make([]string, 0, len(lines))
	for i, line := range lines {
		if i > 0 && !strings.Contains(line, "background-color") {
			filtered = append(filtered, line)
		}
	}
	return strings.Join(filtered, "\n")
}

func HighlightLines(src string, l chroma.Lexer) ([]string, error) {
	l = chroma.Coalesce(l)
	it, err := l.Tokenise(nil, src)
	if err != nil {
		return nil, err
	}
	tokens := it.Tokens()
	lines := chroma.SplitTokensIntoLines(tokens)
	out := make([]string, 0, len(lines))
	for i, toks := range lines {
		f := html.New(
			html.WithClasses(true),
			html.WithPreWrapper(noPreWrapper{}),
			html.WithLineNumbers(true),
			html.BaseLineNumber(i+1),
		)
		var buf bytes.Buffer
		_ = f.Format(&buf, auxStyle, chroma.Literator(toks...))
		out = append(out, buf.String())
	}
	return out, nil
}
