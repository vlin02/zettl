import { invoke } from "@tauri-apps/api/core"

export type Snippet = {
  id: string
  start: number
  preview_html: string
}

export async function listSnippets({
  startId,
  search,
  limit
}: {
  startId: number | null
  search: string
  limit: number
}): Promise<{
  snippets: Snippet[]
  nextId: number | null
}> {
  return invoke("list_snippets", {
    query: {
      start_id: startId,
      search,
      limit
    }
  })
}
