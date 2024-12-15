import { invoke } from "@tauri-apps/api/core"

export type SnippetData = {
  id: string
  start: number
  preview_html: string
}

export type Snippet = {
  id: string
  start: number
  previewHtml: string
}

export async function listSnippets({
  startId,
  search,
  limit
}: {
  startId: number | null
  search: string
  limit: number
}): Promise<{ snippets: Snippet[]; nextId: number | null }> {
  const {
    snippets,
    next_id
  }: {
    snippets: SnippetData[]
    next_id: number | null
  } = await invoke("list_snippets", {
    query: {
      start_id: startId,
      search,
      limit
    }
  })

  return {
    snippets: snippets.map(({ id, start, preview_html }) => {
      return { id, start, previewHtml: preview_html }
    }),
    nextId: next_id
  }
}
