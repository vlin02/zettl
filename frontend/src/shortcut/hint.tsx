import { formatModifier, formatKeyCode } from '@/shortcut'
import { KeyBinding } from 'bindings/zettl/pkg/models'

interface KeyHintProps {
  keyBinding: KeyBinding
}

export const KeyHint = ({ keyBinding }: KeyHintProps) => {
  if (!keyBinding || (!keyBinding.modifiers?.length && !keyBinding.code)) {
    return null
  }

  const keys: string[] = []
  
  if (keyBinding.modifiers && keyBinding.modifiers.length > 0) {
    keyBinding.modifiers.forEach(modifier => {
      keys.push(formatModifier(modifier))
    })
  }
  
  if (keyBinding.code) {
    keys.push(formatKeyCode(keyBinding.code))
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