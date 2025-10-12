import { useRef, useState } from 'react'
import { Snippet } from '../../bindings/zettl/pkg'
import { FindSnippets } from '../../bindings/zettl/service'

const PAGE_SIZE = 100

export type Search = {
  query: string
  snippets: Snippet[]
  selectedIndex: number
}

export function useSearch() {
  const [search, setSearch] = useState<Search | null>(null)
  const pageLockId = useRef(0)
  const searchRef = useRef<Search | null>(null)

  searchRef.current = search

  const loadPage = async (query?: string) => {
    if (query !== undefined) {
      const lockId = ++pageLockId.current
      setSearch(prev => (prev ? { ...prev, query } : { query, snippets: [], selectedIndex: -1 }))

      const snippets = (await FindSnippets(query, 0, PAGE_SIZE)) || []

      if (pageLockId.current !== lockId) return
      pageLockId.current = 0

      setSearch({ query, snippets, selectedIndex: snippets.length > 0 ? 0 : -1 })
    } else {
      const s = searchRef.current
      if (!s || pageLockId.current !== 0) return
      const lockId = ++pageLockId.current

      const before = s.snippets.length ? s.snippets[s.snippets.length - 1].id : 0
      const snippets = (await FindSnippets(s.query, before, PAGE_SIZE)) || []

      if (pageLockId.current !== lockId) return
      pageLockId.current = 0

      setSearch(prev => (prev ? { ...prev, snippets: [...prev.snippets, ...snippets] } : prev))
    }
  }

  const updateIndex = (direction: 'up' | 'down' | number) => {
    if (direction === -1) {
      setSearch(prev => (prev ? { ...prev, selectedIndex: -1 } : null))
      return
    }

    setSearch(prev => {
      if (!prev) return null

      const count = prev.snippets.length
      const toIndex =
        typeof direction === 'number'
          ? direction
          : (prev.selectedIndex === -1 ? (direction === 'down' ? -1 : count) : prev.selectedIndex) +
            (direction === 'down' ? 1 : -1)

      return {
        ...prev,
        selectedIndex: Math.max(0, Math.min(toIndex, count - 1)),
      }
    })
  }

  return { search, loadPage, updateIndex }
}
