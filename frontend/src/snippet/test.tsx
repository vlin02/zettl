import { useEffect, useRef, useState } from 'react'
import type { CSSProperties } from 'react'
import { Input } from '../components/ui/input.tsx'
import { Clipboard, Window, Application } from '@wailsio/runtime'
import { UISettings, SnippetPreview } from '../../bindings/zettl/pkg/models'
import { Search, Settings as SettingsIcon } from 'lucide-react'
import { Button } from '../components/ui/button.tsx'
import { SnippetItem } from './item.tsx'
import { SettingsPanel } from '../settings/panel.tsx'
import { ExpandedView } from './expanded.tsx'
import { detect } from '../detect.ts'
import { List, AutoSizer, CellMeasurer, CellMeasurerCache } from 'react-virtualized'
import { AddSnippet, FindSnippets, GetUISettings, Paste } from '../../bindings/zettl/service'
import { fromKeyboardEvent, shortcutToString } from '../shortcut.ts'

const SCROLL_DELAY = 150
const SCROLL_INTERVAL = 20
const PAGE_SIZE = 100

type Search = {
  query: string
  snippets: SnippetPreview[]
  selectedIndex: number
}

export function Sidebar() {
  const [search, setSearch] = useState<Search | null>(null)
  const [showSettings, setShowSettings] = useState(false)
  const [settings, setSettings] = useState<UISettings | null>(null)

  const pageLockId = useRef(0)
  const queryRef = useRef<HTMLInputElement>(null)
  const searchRef = useRef<Search | null>(null)
  const cancelScrollRef = useRef<(() => void) | null>(null)
  const listRef = useRef<List>(null)

  const cache = useRef(
    new CellMeasurerCache({
      fixedWidth: true,
      defaultHeight: 100,
    }),
  )

  const loadSettings = async () => {
    const settings = await GetUISettings()

    setSettings(settings)
    cache.current.clearAll()
  }

  const loadFirstPage = async (query: string) => {
    const lockId = ++pageLockId.current
    setSearch(prev =>
      prev ? { ...prev, query, selectedIndex: -1 } : { query, snippets: [], selectedIndex: -1 },
    )

    const snippets = (await FindSnippets(query, 0, PAGE_SIZE)) || []

    if (pageLockId.current !== lockId) return
    pageLockId.current = 0

    cache.current.clearAll()
    setSearch({ query, snippets, selectedIndex: -1 })
  }

  const loadNextPage = async (search: Search) => {
    if (!search || pageLockId.current !== 0) return
    const lockId = ++pageLockId.current

    const before = search.snippets.length ? search.snippets[search.snippets.length - 1].id : 0
    const snippets = (await FindSnippets(search.query, before, PAGE_SIZE)) || []

    if (pageLockId.current !== lockId) return
    pageLockId.current = 0

    setSearch(prev => (prev ? { ...prev, snippets: [...prev.snippets, ...snippets] } : prev))
  }

  const selectIndex = (direction: 'up' | 'down' | number) =>
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

  const deselectIndex = () => setSearch(prev => (prev ? { ...prev, selectedIndex: -1 } : null))

  const onCopy = async (text: string, paste: boolean = false) => {
    Window.Hide()
    await Clipboard.SetText(text)
    if (paste) await Paste()
  }

  const renderRow = ({
    index,
    key,
    style,
    parent,
  }: {
    index: number
    key: string
    style: CSSProperties
    parent: any
  }) => {
    return (
      <CellMeasurer
        key={key}
        cache={cache.current}
        parent={parent}
        columnIndex={0}
        rowIndex={index}
      >
        {({ registerChild }) => (
          <div ref={registerChild} style={style}>
            <div className="px-2 pb-2">
              <SnippetItem
                snippet={search!.snippets[index]}
                isSelected={search!.selectedIndex === index}
                onClick={() => {
                  search?.selectedIndex === index ? deselectIndex() : selectIndex(index)
                }}
                onCopy={onCopy}
                fontSize={settings!.font_size}
              />
            </div>
          </div>
        )}
      </CellMeasurer>
    )
  }

  const startScroll = (direction: 'up' | 'down') => {
    if (cancelScrollRef.current) {
      cancelScrollRef.current()
    }

    selectIndex(direction)

    const delayTimeout = setTimeout(() => {
      const interval = setInterval(() => selectIndex(direction), SCROLL_INTERVAL)
      cancelScrollRef.current = () => clearInterval(interval)
    }, SCROLL_DELAY)

    cancelScrollRef.current = () => clearTimeout(delayTimeout)
  }

  const stopScroll = () => {
    if (cancelScrollRef.current) {
      cancelScrollRef.current()
      cancelScrollRef.current = null
    }
  }

  useEffect(() => {
    loadSettings()
    loadFirstPage('')
  }, [])

  useEffect(() => {
    let lastText: string | undefined
    let lock = false

    const id = window.setInterval(async () => {
      if (lock) return
      lock = true
      const visible = document.visibilityState === 'visible'
      try {
        const text = await Clipboard.Text()
        if (text !== lastText) {
          const lang = await detect(text)
          await AddSnippet(text, lang)
          await loadFirstPage('')
          if (lastText !== undefined && !visible) await Window.Hide()
        }
        lastText = text
      } finally {
        lock = false
      }
    }, 200)

    return () => clearInterval(id)
  }, [])

  useEffect(() => {
    searchRef.current = search
  }, [search])

  useEffect(() => {
    let lastDir: 'up' | 'down' | null = null

    const onKeyDown = (e: KeyboardEvent) => {
      const s = shortcutToString(fromKeyboardEvent(e))

      switch (s) {
        case 'Meta+ArrowUp': {
          e.preventDefault()
          const s = searchRef.current
          if (s && s.snippets.length > 0) selectIndex(0)
          return
        }
        case 'Meta+KeyQ': {
          e.preventDefault()
          Application.Quit()
          return
        }
        case 'Meta+KeyL': {
          e.preventDefault()
          queryRef.current?.focus()
          return
        }
        case 'Escape': {
          e.preventDefault()
          Window.Hide()
          return
        }
        case 'Meta+KeyC':
        case 'Enter': {
          const s = searchRef.current
          if (s && s.selectedIndex >= 0) {
            e.preventDefault()
            const { content } = s.snippets[s.selectedIndex]
            onCopy(content, false)
          }
          return
        }
        case 'Meta+Enter': {
          const s = searchRef.current
          if (s && s.selectedIndex >= 0) {
            e.preventDefault()
            const { content } = s.snippets[s.selectedIndex]
            onCopy(content, true)
          }
          return
        }
        case 'ArrowDown': {
          e.preventDefault()
          if (lastDir !== 'down') {
            lastDir = 'down'
            startScroll('down')
          }
          return
        }
        case 'ArrowUp': {
          e.preventDefault()
          if (lastDir !== 'up') {
            lastDir = 'up'
            startScroll('up')
          }
          return
        }
        default:
          return
      }
    }

    const onKeyUp = (e: KeyboardEvent) => {
      const s = shortcutToString(fromKeyboardEvent(e))

      switch (s) {
        case 'ArrowDown':
          if (lastDir === 'down') {
            lastDir = null
            stopScroll()
          }
          break
        case 'ArrowUp':
          if (lastDir === 'up') {
            lastDir = null
            stopScroll()
          }
          break
        default:
          break