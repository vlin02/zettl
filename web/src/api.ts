import { invoke } from "@tauri-apps/api/core"

export type Snippet = {
  id: number
  start: number
  preview_html: string
}

type User = {
  popup_width: number
  popup_height: number
  popup_transparent: boolean
  theme_id: number
  crop_whitespace: boolean
}

type ThemeListing = {
  id: number
  name: string
  can_delete: boolean
  active: boolean
  preview_colors: { r: number; g: number; b: number; a: number }[]
}

export type Settings = { user: User; themes: ThemeListing[] }

export async function listSnippets(query: {
  start_id: number | null
  search: string
  limit: number
}): Promise<{ snippets: Snippet[]; nextId: number | null }> {
  return invoke("list_snippets", {
    query
  })
}

export async function copySnippet(id: number) {
  await invoke("copy_snippet", { id })
}

export async function getSettings(): Promise<Settings> {
  return invoke("get_settings")
}

export async function updateUser(user: User) {
  await invoke("update_user", user)
}

export async function setActiveTheme(id: number) {
  await invoke("set_active_theme", { id })
}

export async function importTheme(name: string, text: string) {
  await invoke("import_theme", {
    name,
    text
  })
}

export async function deleteTheme(id: number) {
  await invoke("delete_theme", { id })
}

export async function previewTheme(tmPlist: string) {
  await invoke("preview_theme", { tm_plist: tmPlist })
}

export async function loadActiveTheme(tmPlist: string) {
  await invoke("load_active_theme", { tm_plist: tmPlist })
}

export async function closeWindow() {
  await invoke("close_window")
}
