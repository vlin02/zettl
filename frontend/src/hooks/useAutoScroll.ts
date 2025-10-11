import { useRef } from 'react'

const SCROLL_DELAY = 150
const SCROLL_INTERVAL = 25

export function useAutoScroll(onTick: (direction: 'up' | 'down') => void) {
  const cancelScrollRef = useRef<(() => void) | null>(null)

  const startScroll = (direction: 'up' | 'down') => {
    if (cancelScrollRef.current) {
      cancelScrollRef.current()
    }

    onTick(direction)

    const delayTimeout = setTimeout(() => {
      const interval = setInterval(() => onTick(direction), SCROLL_INTERVAL)
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

  return { startScroll, stopScroll }
}

