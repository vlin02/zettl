import { useEffect, useRef, useState } from 'react'
import type { CSSProperties } from 'react'
import { Input } from '../components/ui/input.tsx'
import { Clipboard, Window, Application } from '@wailsio/runtime'
import { UISettings, SnippetPreview } from '../../bindings/zettl/pkg/models'
import { Search } from './language.tsx'
import { Settings as SettingsIcon } from 'lucide-react'
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
    const s = await GetUISettings()

    setSettings(s)
    cache.current.clearAll()
  }

  const loadFirstPage = async (query: string) => {
    console.log(query)
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
    await navigator.clipboard.writeText(text)
    await Window.Hide()
    if (paste) await Paste(text, true)
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
    if (!search || !search.snippets[index] || !settings) return null

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
                snippet={search.snippets[index]}
                isSelected={search.selectedIndex === index}
                onClick={() => (index: number) =>
                  search?.selectedIndex === index ? deselectIndex() : selectIndex(index)}
                onCopy={onCopy}
                fontSize={settings.font_size}
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
    queryRef.current?.focus()
  }, [])

  useEffect(() => {
    const onVis = () => {
      queryRef.current?.focus()
      queryRef.current?.select()
    }
    document.addEventListener('visibilitychange', onVis)
    return () => document.removeEventListener('visibilitychange', onVis)
  }, [])

  useEffect(() => {
    let lastClip = ''
    let intervalId: number | undefined
    ;(async () => {
      lastClip = await Clipboard.Text()

      intervalId = window.setInterval(async () => {
        if (!search) return
        const clip = (await Clipboard.Text()) || ''
        if (clip && clip !== lastClip) {
          lastClip = clip
          const lang = await detect(clip)
          await AddSnippet(clip, lang)
          Window.Hide()
        }
      }, 200)
    })()

    return () => {
      if (intervalId !== undefined) clearInterval(intervalId)
    }
  }, [])

  useEffect(() => {
    searchRef.current = search
  }, [search])

  useEffect(() => {
    let lastDir: 'up' | 'down' | null = null

    const onKeyDown = (e: KeyboardEvent) => {
      const s = shortcutToString(fromKeyboardEvent(e))
      console.log(s)

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
      }
    }

    window.addEventListener('keydown', onKeyDown)
    window.addEventListener('keyup', onKeyUp)
    return () => {
      window.removeEventListener('keydown', onKeyDown)
      window.removeEventListener('keyup', onKeyUp)
      stopScroll()
    }
  }, [])

  useEffect(() => {
    if (search && search.selectedIndex !== -1) {
      queryRef.current?.blur()
      if (listRef.current) listRef.current.scrollToRow(search.selectedIndex)
    }
  }, [search?.selectedIndex])

  if (!settings || !search) return null

  return (
    <div className="h-full flex overflow-hidden">
      <style dangerouslySetInnerHTML={{ __html: settings.style.css }} />
      <div className="w-[50ch] flex flex-col min-h-0 overflow-hidden">
        {showSettings ? (
          <div className="bg-background h-full">
            <SettingsPanel
              isOpen={showSettings}
              onClose={() => setShowSettings(false)}
              settings={settings}
              onRefetch={loadSettings}
            />
          </div>
        ) : (
          <>
            <div className="p-3 flex items-center gap-2">
              <div className="relative flex-1">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder="Search..."
                  value={search.query}
                  onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
                    loadFirstPage(e.target.value)
                  }}
                  onFocus={() => {
                    if (search.selectedIndex >= 0) deselectIndex()
                  }}
                  className="pl-10 h-8 text-sm bg-background/50 border-border/50"
                  id="zettl-focus-input"
                  ref={queryRef}
                />
              </div>

              <Button
                type="button"
                onClick={() => {
                  deselectIndex()
                  setShowSettings(true)
                }}
                variant="secondary"
                size="icon"
                className="h-8 w-8"
                title="Settings"
              >
                <SettingsIcon className="h-4 w-4 text-muted-foreground" />
              </Button>
            </div>
            <div className="flex-1 overflow-hidden">
              {search.snippets.length > 0 ? (
                <div className="h-full">
                  <AutoSizer>
                    {({ height, width }) => (
                      <List
                        ref={listRef}
                        height={height}
                        width={width}
                        rowCount={search.snippets.length}
                        rowHeight={cache.current.rowHeight}
                        deferredMeasurementCache={cache.current}
                        rowRenderer={renderRow}
                        onScroll={({ scrollTop, scrollHeight, clientHeight }) => {
                          const dist = scrollHeight - scrollTop - clientHeight
                          if (dist <= clientHeight * 2 && pageLockId.current === 0) {
                            loadNextPage(search)
                          }
                        }}
                        overscanRowCount={5}
                      />
                    )}
                  </AutoSizer>
                </div>
              ) : search.query.length === 0 ? (
                <div className="text-center py-12 text-muted-foreground">
                  <Search className="h-8 w-8 mx-auto mb-3 opacity-50" />
                  <p className="text-sm">No snippets found</p>
                  <p className="text-xs mt-1">Try a different search term</p>
                </div>
              ) : null}
            </div>
          </>
        )}
      </div>

      <div
        className={`bg-background transition-all duration-300 ${
          search.selectedIndex >= 0 ? 'w-[80ch] opacity-100' : 'w-0 opacity-0 pointer-events-none'
        }`}
      >
        {search.selectedIndex >= 0 && search.snippets[search.selectedIndex] && (
          <ExpandedView
            snippet={search.snippets[search.selectedIndex]}
            fontSize={settings.font_size}
          />
        )}
      </div>
    </div>
  )
}
