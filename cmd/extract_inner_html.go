package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	// Read the HTML file
	content, err := os.ReadFile("chroma_output.html")
	if err != nil {
		panic(err)
	}

	htmlStr := string(content)

	// Extract just the inner content (between <code> and </code>)
	start := strings.Index(htmlStr, "<code>")
	end := strings.Index(htmlStr, "</code>")

	if start != -1 && end != -1 {
		innerHTML := htmlStr[start+6 : end]

		// Save to file
		err = os.WriteFile("chroma_inner.html", []byte(innerHTML), 0644)
		if err != nil {
			panic(err)
		}

		fmt.Println("Inner HTML saved to chroma_inner.html")
		fmt.Printf("Size: %d bytes\n", len(innerHTML))
	} else {
		fmt.Println("Could not find code tags")
	}
}
