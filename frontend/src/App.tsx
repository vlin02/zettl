import { useEffect, useRef, useState } from 'react'
import { Sidebar } from './snippet/sidebar'
import { ThemeProvider } from 'next-themes'
import { SetWidth, FrontendReady } from '../bindings/zettl/service'

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
