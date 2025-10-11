import { useEffect, useRef, useState } from 'react'
import { Input } from '../components/ui/input.tsx'
import { Clipboard, Window, Application } from '@wailsio/runtime'
import { UISettings } from '../../bindings/zettl/pkg/models.ts'
import { Search, Settings as SettingsIcon } from 'lucide-react'
import { Button } from '../components/ui/button.tsx'
import { SettingsPanel } from '../settings/panel.tsx'
import { ExpandedView } from './expanded.tsx'
import { VirtualizedList } from './list.tsx'
import { SnippetItem } from './item.tsx'
import { GetUISettings, Paste } from '../../bindings/zettl/service.ts'
import { fromKeyboardEvent, shortcutToString } from '../shortcut/index.ts'
import { useAutoScroll } from '../hooks/useAutoScroll.ts'
import { useSearch } from '../hooks/useSearch.ts'

export function Sidebar() {
  const [showSettings, setShowSettings] = useState(false)
  const [settings, setSettings] = useState<UISettings | null>(null)

  const queryRef = useRef<HTMLInputElement>(null)
  const searchRef = useRef(null as ReturnType<typeof useSearch>['search'])

  const { search, loadPage, updateIndex } = useSearch()
  const { startScroll, stopScroll } = useAutoScroll(updateIndex)

  const loadSettings = async () => {
    const settings = await GetUISettings()
    setSettings(settings)
  }

  const onCopy = async (text: string, paste: boolean = false) => {
    Window.Hide()
    await Clipboard.SetText(text)
    if (paste) await Paste()
  }

  const copySelected = (paste: boolean) => {
    const s = searchRef.current
    if (s && s.selectedIndex >= 0) {
      const { content } = s.snippets[s.selectedIndex]
      onCopy(content, paste)
    }
  }

  useEffect(() => {
    searchRef.current = search
  }, [search])

  useEffect(() => {
    loadSettings()
    loadPage('')
  }, [])

  useEffect(() => {
    let lastDir: 'up' | 'down' | null = null

    const shortcuts: Record<string, () => void> = {
      'Meta+ArrowUp': () => {
        const s = searchRef.current
        if (s && s.snippets.length > 0) updateIndex(0)
      },
      'Escape': () => Window.Hide(),
      'Meta+KeyL': () => queryRef.current?.focus(),
      'Meta+KeyQ': () => Application.Quit(),
      'Meta+KeyC': () => copySelected(false),
      'Enter': () => copySelected(false),
      'Meta+Enter': () => copySelected(true),
      'ArrowDown': () => {
        if (lastDir !== 'down') {
          lastDir = 'down'
          startScroll('down')
        }
      },
      'ArrowUp': () => {
        if (lastDir !== 'up') {
          lastDir = 'up'
          startScroll('up')
        }
      },
    }

    const keyUpHandlers: Record<string, () => void> = {
      'ArrowDown': () => {
        if (lastDir === 'down') {
          lastDir = null
          stopScroll()
        }
      },
      'ArrowUp': () => {
        if (lastDir === 'up') {
          lastDir = null
          stopScroll()
        }
      },
    }

    const onKeyDown = (e: KeyboardEvent) => {
      const handler = shortcuts[shortcutToString(fromKeyboardEvent(e))]
      if (handler) {
        e.preventDefault()
        handler()
      }
    }

    const onKeyUp = (e: KeyboardEvent) => {
      const handler = keyUpHandlers[shortcutToString(fromKeyboardEvent(e))]
      if (handler) handler()
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
    const onVisibility = () => {
      if (document.visibilityState === 'visible') {
        queryRef.current?.focus()
      }
    }

    document.addEventListener('visibilitychange', onVisibility)
    return () => document.removeEventListener('visibilitychange', onVisibility)
  }, [])

  if (!settings || !search) return null

  return (
    <div className="h-full flex overflow-hidden" onClick={() => queryRef.current?.focus()}>
      <style dangerouslySetInnerHTML={{ __html: settings.style.css }} />
      <div className="w-[50ch] flex flex-col min-h-0 overflow-hidden">
        {showSettings ? (
          <div className="bg-background h-full">
            <SettingsPanel
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
                    loadPage(e.target.value)
                  }}
                  className="pl-10 h-8 text-sm bg-background/50 border-border/50 focus:outline-none focus:ring-0 focus:border-border/50 focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-offset-0"
                  id="zettl-focus-input"
                  ref={queryRef}
                  autoFocus
                />
              </div>

              <Button
                type="button"
                onClick={() => {
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
                <VirtualizedList
                  items={search.snippets}
                  selectedIndex={search.selectedIndex}
                  renderItem={(snippet, isSelected, index) => (
                    <div className="px-2 pb-2">
                      <SnippetItem
                        snippet={snippet}
                        isSelected={isSelected}
                        onClick={() => updateIndex(isSelected ? -1 : index)}
                        onCopy={onCopy}
                        fontSize={settings.font_size}
                      />
                    </div>
                  )}
                  onLoadMore={() => loadPage()}
                />
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
      <div className="w-[80ch] bg-background">
        {search.selectedIndex >= 0 && search.snippets[search.selectedIndex] ? (
          <ExpandedView
            snippet={search.snippets[search.selectedIndex]}
            fontSize={settings.font_size}
          />
        ) : (
          <div className="h-full flex items-center justify-center text-muted-foreground">
            <div className="text-center">
              <div className="text-sm opacity-50">No snippet selected</div>
              <div className="text-xs mt-1 opacity-30">Select a snippet to view details</div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
