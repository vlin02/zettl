import { useEffect, useRef, useState } from 'react'
import type { CSSProperties } from 'react'
import { Input } from '../components/ui/input'
import { Clipboard, Window } from '@wailsio/runtime'
import { UISettings, SnippetPreview } from '../../bindings/zettl/pkg/models'
import { Search } from './clipboard-icons.tsx'
import { Sun, Moon, Settings as SettingsIcon } from 'lucide-react'
import { SnippetItem } from './snippet-item'
import { SettingsPanel } from '../settings/panel'
import { SnippetDetail } from './snippet-detail'
import { detect } from '../detect'
import { List, AutoSizer, CellMeasurer, CellMeasurerCache } from 'react-virtualized'
import { AddSnippet, FindSnippets, GetUISettings, SetUITheme } from '../../bindings/zettl/service'

const SCROLL_DELAY = 100
const SCROLL_INTERVAL = 20

export function ClipboardSidebar() {
  const [page, setPage] = useState<{
    query: string
    items: SnippetPreview[]
    selectedIndex: number
  } | null>(null)
  const [showSettings, setShowSettings] = useState(false)
  const [settings, setSettings] = useState<UISettings | null>(null)

  const pageLockId = useRef(0)
  const queryRef = useRef<HTMLInputElement>(null)
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

    document.documentElement.classList.toggle('dark', s.theme === 'dark')
    setSettings(s)

    cache.current.clearAll()
  }

  const select = (direction: 'up' | 'down' | number) =>
    setPage(prev => {
      if (!prev) return null

      let nextIndex: number
      if (typeof direction === 'number') {
        nextIndex = direction
      } else {
        const start =
          prev.selectedIndex === -1
            ? direction === 'down'
              ? -1
              : prev.items.length
            : prev.selectedIndex
        nextIndex = start + (direction === 'down' ? 1 : -1)
      }

      return {
        ...prev,
        selectedIndex: Math.max(0, Math.min(nextIndex, prev.items.length - 1)),
      }
    })

  const deselect = () => setPage(prev => (prev ? { ...prev, selectedIndex: -1 } : null))

  const loadPage = async (mode: 'reset' | 'append') => {
    if (mode === 'append' && pageLockId.current !== 0) return

    const currentLockId = ++pageLockId.current
    const query = page?.query || ''
    const before =
      mode === 'append' && page?.items.length ? page.items[page.items.length - 1].id : 0

    const rows = (await FindSnippets('a', before, 100)) || []

    if (pageLockId.current !== currentLockId) return

    if (mode === 'reset') {
      cache.current.clearAll()
      setPage({ query, items: rows, selectedIndex: -1 })
    } else if (page) {
      setPage({ ...page, items: [...page.items, ...rows] })
    }
    pageLockId.current = 0
  }

  const onCopy = async (text: string) => {
    await navigator.clipboard.writeText(text)
    Window.Hide()
  }

  const handleSnippetClick = (index: number) =>
    page?.selectedIndex === index ? deselect() : select(index)

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
    if (!page || !page.items[index] || !settings) return null

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
                snippet={page.items[index]}
                isSelected={page.selectedIndex === index}
                onClick={() => handleSnippetClick(index)}
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

    select(direction)

    const delayTimeout = setTimeout(() => {
      const interval = setInterval(() => select(direction), SCROLL_INTERVAL)
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
    loadPage('reset')
    queryRef.current?.focus()
  }, [])

  const handleQueryChange = async (newQuery: string) => {
    const currentLockId = ++pageLockId.current

    setPage(prev =>
      prev ? { ...prev, query: newQuery } : { query: newQuery, items: [], selectedIndex: -1 },
    )

    if (newQuery.length > 0 && newQuery.length < 3) {
      setPage(prev => (prev ? { ...prev, items: [], selectedIndex: -1 } : null))
      pageLockId.current = 0
      return
    }

    const rows = (await FindSnippets(newQuery, 0, 100)) || []
    if (pageLockId.current === currentLockId) {
      cache.current.clearAll()
      setPage({ query: newQuery, items: rows, selectedIndex: -1 })
      pageLockId.current = 0
    }
  }

  useEffect(() => {
    let lastClip = ''
    let mounted = true

    const checkClipboard = async () => {
      if (!mounted) return

      const currentClip = await Clipboard.Text()

      if (currentClip && currentClip !== lastClip) {
        lastClip = currentClip
        const lang = await detect(currentClip)
        await AddSnippet(currentClip, lang)
        loadPage('reset')
        Window.Hide()
      }
    }

    Clipboard.Text()
      .then(async text => {
        if (mounted) lastClip = text || ''
      })
      .catch(() => {})

    const interval = setInterval(checkClipboard, 200)

    return () => {
      mounted = false
      clearInterval(interval)
    }
  }, [])

  useEffect(() => {
    if (showSettings) {
      stopScroll()
      return
    }

    let scrollDirection: 'up' | 'down' | null = null

    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key !== 'ArrowDown' && e.key !== 'ArrowUp') return

      e.preventDefault()
      const direction = e.key === 'ArrowDown' ? 'down' : 'up'

      if (scrollDirection !== direction) {
        scrollDirection = direction
        startScroll(direction)
      }
    }

    const onKeyUp = (e: KeyboardEvent) => {
      if (e.key !== 'ArrowDown' && e.key !== 'ArrowUp') return
      const direction = e.key === 'ArrowDown' ? 'down' : 'up'
      if (scrollDirection === direction) {
        scrollDirection = null
        stopScroll()
      }
    }

    window.addEventListener('keydown', onKeyDown)
    window.addEventListener('keyup', onKeyUp)

    return () => {
      window.removeEventListener('keydown', onKeyDown)
      window.removeEventListener('keyup', onKeyUp)
      stopScroll()
    }
  }, [showSettings])

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.key === 'l' || e.key === 'L') && e.metaKey) {
        e.preventDefault()
        queryRef.current?.focus()
        queryRef.current?.select()
      } else if (e.key === 'Escape') {
        e.preventDefault()
        Window.Hide()
      } else if (e.key === 'Enter' && page && page.selectedIndex >= 0) {
        e.preventDefault()
        onCopy(page.items[page.selectedIndex].content)
      }
    }

    window.addEventListener('keydown', onKeyDown)

    return () => {
      window.removeEventListener('keydown', onKeyDown)
    }
  }, [page?.selectedIndex, showSettings])

  useEffect(() => {
    if (page && page.selectedIndex >= 0 && listRef.current) {
      listRef.current.scrollToRow(page.selectedIndex)
    }
  }, [page?.selectedIndex, page?.items])

  if (!settings || !page) return null

  const onToggleTheme = async () => {
    await SetUITheme(settings.theme === 'dark' ? 'light' : 'dark')
    await loadSettings()
  }

  const bgColor = settings.theme === 'light' ? settings.light_bg_color : settings.dark_bg_color
  const bgStyle = showSettings ? { backgroundColor: bgColor } : { backgroundColor: bgColor + 'F2' } // 95% opacity

  return (
    <div className="h-full backdrop-blur-xl flex relative transition-all overflow-hidden">
      <style dangerouslySetInnerHTML={{ __html: settings.style.css }} />
      <div className="w-[60ch] flex flex-col min-h-0 overflow-hidden" style={bgStyle}>
        {showSettings ? (
          <SettingsPanel
            isOpen={showSettings}
            onClose={() => setShowSettings(false)}
            settings={settings}
            onRefetch={loadSettings}
          />
        ) : (
          <>
            <div className="p-3 flex items-center gap-2">
              <button
                type="button"
                onClick={onToggleTheme}
                title={`${settings.theme === 'dark' ? 'dark' : 'light'} mode`}
                className="relative h-8 w-8 rounded-full border border-border/50 hover:bg-accent/30 flex items-center justify-center overflow-hidden"
              >
                <Sun
                  className={`absolute h-4 w-4 transition-all duration-300 ${
                    settings.theme === 'light'
                      ? 'opacity-100 scale-100 rotate-0'
                      : 'opacity-0 scale-0 -rotate-90'
                  }`}
                />
                <Moon
                  className={`absolute h-4 w-4 transition-all duration-300 ${
                    settings.theme === 'dark'
                      ? 'opacity-100 scale-100 rotate-0'
                      : 'opacity-0 scale-0 rotate-90'
                  }`}
                />
              </button>

              <div className="relative flex-1">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder="Search snippets..."
                  value={page.query}
                  onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                    handleQueryChange(e.target.value)
                  }
                  className="pl-10 h-8 text-sm bg-background/50 border-border/50 focus:border-border focus:ring-1 focus:ring-ring"
                  id="query"
                  ref={queryRef}
                />
              </div>

              <button
                type="button"
                onClick={() => {
                  deselect()
                  setShowSettings(true)
                }}
                className="h-8 w-8 p-0 rounded border border-border/50 hover:bg-accent/30 flex items-center justify-center"
                title="Settings"
              >
                <SettingsIcon className="h-4 w-4 text-muted-foreground" />
              </button>
            </div>

            <div className="flex-1 overflow-hidden">
              {page.items.length > 0 ? (
                <div className="h-full pt-2">
                  <AutoSizer>
                    {({ height, width }) => (
                      <List
                        ref={listRef}
                        height={height}
                        width={width}
                        rowCount={page.items.length}
                        rowHeight={cache.current.rowHeight}
                        deferredMeasurementCache={cache.current}
                        rowRenderer={renderRow}
                        onScroll={({ scrollTop, scrollHeight, clientHeight }) => {
                          const dist = scrollHeight - scrollTop - clientHeight
                          if (dist <= clientHeight * 2 && pageLockId.current === 0) {
                            loadPage('append')
                          }
                        }}
                        overscanRowCount={5}
                      />
                    )}
                  </AutoSizer>
                </div>
              ) : page.query.length === 0 ? (
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
        className={`bg-background/95 transition-all duration-300 ease-in-out ${
          page.selectedIndex >= 0 ? 'w-[120ch] opacity-100' : 'w-0 opacity-0 pointer-events-none'
        }`}
      >
        {page.selectedIndex >= 0 && page.items[page.selectedIndex] && (
          <SnippetDetail
            snippet={page.items[page.selectedIndex]}
            onCopy={onCopy}
            onClose={deselect}
            fontSize={settings.font_size}
          />
        )}
      </div>
    </div>
  )
}
