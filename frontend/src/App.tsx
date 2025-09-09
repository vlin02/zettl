import { useEffect, useRef, useState } from 'react'
import { Sidebar } from './snippet/sidebar'
import { Window, Events } from '@wailsio/runtime'
import { ThemeProvider } from 'next-themes'

function App() {
  const rootRef = useRef<HTMLDivElement>(null)
  const [sidebarKey, setSidebarKey] = useState(0)

  const heightRef = useRef(0)
  const [height, setHeight] = useState(0)

  useEffect(() => {
    const unsub = Events.On('windowHeight', ev => {
      const h: number = ev.data[0]
      if (h !== heightRef.current) {
        heightRef.current = h
        setHeight(h)
      }
    })
    return () => unsub()
  }, [])

  useEffect(() => {
    if (!rootRef.current) return
    let prevW = 0
    const ro = new ResizeObserver(entries => {
      const w = Math.round(entries[0].contentRect.width)
      if (w !== prevW && heightRef.current) {
        prevW = w
        Window.SetSize(w, heightRef.current)
      }
    })
    ro.observe(rootRef.current)
    return () => ro.disconnect()
  }, [height])

  useEffect(() => {
    const onVisibility = () => {
      if (document.visibilityState === 'hidden') {
        setSidebarKey(k => 1 - k)
      }
    }
    document.addEventListener('visibilitychange', onVisibility)
    return () => document.removeEventListener('visibilitychange', onVisibility)
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
