import { useEffect, useRef, useState } from 'react'
import { Sidebar } from './snippet/sidebar'
import { ThemeProvider } from 'next-themes'
import { SetWidth, FrontendReady, AddSnippet } from '../bindings/zettl/service'
import { Clipboard, Window } from '@wailsio/runtime'
import { detect } from './detect'

function App() {
  const rootRef = useRef<HTMLDivElement>(null)
  const [sidebarKey, setSidebarKey] = useState(0)
  const widthRef = useRef(0)
  const readyRef = useRef(false)

  useEffect(() => {
    if (!rootRef.current) return

    const ro = new ResizeObserver(entries => {
      widthRef.current = Math.round(entries[0].contentRect.width)
      SetWidth(widthRef.current)
      if (!readyRef.current) {
        setTimeout(() => {
          readyRef.current = true
          FrontendReady()
        }, 100)
      }
    })

    ro.observe(rootRef.current)

    return () => ro.disconnect()
  }, [sidebarKey])

  useEffect(() => {
    const onVisibility = () => {
      if (document.visibilityState === 'hidden') {
        setSidebarKey(k => 1 - k)
      }
    }
    document.addEventListener('visibilitychange', onVisibility)
    return () => document.removeEventListener('visibilitychange', onVisibility)
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
        if (lastText !== undefined && text !== lastText) {
          const lang = await detect(text)
          await AddSnippet(text, lang)
          setSidebarKey(k => k + 1)
          if (!visible) await Window.Hide()
        }
        lastText = text
      } finally {
        lock = false
      }
    }, 200)

    return () => clearInterval(id)
  }, [])

  return (
    <ThemeProvider attribute="class" enableSystem>
      <div className="overflow-x-hidden">
        <div className="h-screen w-fit" ref={rootRef}>
          <Sidebar key={sidebarKey} />
        </div>
      </div>
    </ThemeProvider>
  )
}

export default App
