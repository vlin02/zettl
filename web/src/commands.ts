import { invoke } from "@tauri-apps/api/core"

type Snippet = {
  content: string
  html: string
}

export async function listSnippets(req: { search: string }): Promise<Snippet[]> {
  return invoke("list_snippets", req)
}
