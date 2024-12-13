import { invoke } from "@tauri-apps/api/core"

export type Snippet = {
  content: string
  preview_html: string
}

export async function listSnippets(query: { search: string }): Promise<Snippet[]> {
  return invoke("list_snippets", { query })
}
