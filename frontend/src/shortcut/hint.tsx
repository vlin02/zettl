import { formatModifier, formatKeyCode } from '@/shortcut'
import { Shortcut } from 'bindings/zettl/pkg/models'

interface KeyHintProps {
  hotkey: Shortcut
}

export const KeyHint = ({ hotkey }: KeyHintProps) => {
  if (!hotkey || (!hotkey.modifiers?.length && !hotkey.code)) {
    return null
  }

  const keys: string[] = []
  
  if (hotkey.modifiers && hotkey.modifiers.length > 0) {
    hotkey.modifiers.forEach(modifier => {
      keys.push(formatModifier(modifier))
    })
  }
  
  if (hotkey.code) {
    keys.push(formatKeyCode(hotkey.code))
  }

  return (
    <div className="flex items-center gap-1">
      {keys.map((key, index) => (
        <kbd 
          key={index}
          className="px-1.5 py-0.5 text-xs bg-muted rounded font-mono font-medium min-w-fit text-center"
        >
          {key}
        </kbd>
      ))}
    </div>
  )
}