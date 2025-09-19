package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/alecthomas/chroma/v2/formatters/html"
	"github.com/alecthomas/chroma/v2/lexers"
	"github.com/alecthomas/chroma/v2/styles"
)

func main() {
	// Generate 10,000 lines, each with 'A' repeated 100 times
	var lines []string
	lineContent := strings.Repeat("A", 100)

	for i := 0; i < 10000; i++ {
		lines = append(lines, lineContent)
	}

	content := strings.Join(lines, "\n")

	// Get the fallback lexer (plain text)
	lexer := lexers.Fallback

	// Get the style (you can change this to any Chroma style)
	style := styles.Fallback

	// Create HTML formatter with inline styles
	formatter := html.New(
		html.WithClasses(false), // Use inline styles instead of classes
		html.TabWidth(4),
	)

	// Create an iterator for the tokens
	iterator, err := lexer.Tokenise(nil, content)
	if err != nil {
		panic(err)
	}

	// Format the tokens to HTML
	var htmlOutput strings.Builder
	err = formatter.Format(&htmlOutput, style, iterator)
	if err != nil {
		panic(err)
	}

	// Write to file
	err = os.WriteFile("chroma_output.html", []byte(htmlOutput.String()), 0644)
	if err != nil {
		panic(err)
	}

	fmt.Println("HTML output saved to chroma_output.html")
}
