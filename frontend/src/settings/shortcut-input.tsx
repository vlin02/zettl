import { useEffect, useRef, useState } from 'react'
import type { KeyboardEvent } from 'react'
import { HotkeyEvent } from '../../bindings/zettl/pkg/models'

function codeToDisplayName(code: string): string {
  if (code === 'Space') return 'Space'
  if (code.startsWith('Key')) return code.slice(3)
  if (code.startsWith('Digit')) return code.slice(5)
  if (['Enter', 'Escape', 'Backspace', 'Tab'].includes(code)) return code
  if (code.startsWith('Arrow') || /^F\d+$/.test(code)) return code
  return code
}

const MODIFIER_MAP: Record<string, string> = {
  Meta: 'Cmd',
  Control: 'Ctrl',
  Alt: 'Opt',
  Shift: 'Shift',
}

export function formatHotkeyEvent({ modifiers, code }: HotkeyEvent): string {
  const prettyMods = modifiers.map((m: string) => MODIFIER_MAP[m] || m)
  const prettyCode = codeToDisplayName(code)
  const parts = [...prettyMods, ...(code ? [prettyCode] : [])]
  return parts.join('+')
}

const MODIFIER_CODES = [
  'MetaLeft',
  'MetaRight',
  'ControlLeft',
  'ControlRight',
  'ShiftLeft',
  'ShiftRight',
  'AltLeft',
  'AltRight',
]
function getModifiers(e: KeyboardEvent) {
  const mods: string[] = []
  if (e.metaKey) mods.push('Meta')
  if (e.shiftKey) mods.push('Shift')
  if (e.ctrlKey) mods.push('Control')
  if (e.altKey) mods.push('Alt')
  return mods
}

export function ShortcutInput({
  event,
  onSubmit,
}: {
  event: HotkeyEvent
  onSubmit: (event: HotkeyEvent) => void
}) {
  const [pending, setPending] = useState<HotkeyEvent | null>(null)
  const buttonRef = useRef<HTMLButtonElement>(null)

  const onKeyDown = (e: KeyboardEvent) => {
    e.preventDefault()
    const mods = getModifiers(e)
    const isModifierKey = MODIFIER_CODES.includes(e.code)
    const newShortcut = new HotkeyEvent({
      modifiers: mods,
      code: isModifierKey ? '' : e.code,
    })
    setPending(newShortcut)

    if (!isModifierKey) {
      onSubmit(newShortcut)
    }
  }

  useEffect(() => {
    setPending(null)
    buttonRef.current?.blur()
  }, [event])

  const onBlur = () => {
    setPending(null)
  }

  const onFocus = () => setPending(new HotkeyEvent())

  const label = pending ? formatHotkeyEvent(pending) || 'Press shortcut…' : formatHotkeyEvent(event)

  return (
    <button
      ref={buttonRef}
      onKeyDown={onKeyDown}
      onBlur={onBlur}
      onFocus={onFocus}
      tabIndex={0}
      className={`h-8 px-3 rounded text-sm border w-40 text-left ${
        pending ? 'bg-background/50 border-foreground' : 'bg-transparent border-input'
      }`}
    >
      {label}
    </button>
  )
}
