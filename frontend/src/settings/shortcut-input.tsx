import { useEffect, useRef, useState } from 'react'
import type { KeyboardEvent } from 'react'
import { HotkeyEvent } from '../../bindings/zettl/pkg/models'
import { KeyHint } from '@/settings/key-hint'

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

    const shortcut = new HotkeyEvent({
      modifiers: mods,
      code: isModifierKey ? '' : e.code,
    })

    setPending(shortcut)
    if (shortcut.code) {
      onSubmit(shortcut)
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

  const displayEvent = pending || event
  const hasShortcut = displayEvent && (displayEvent.modifiers?.length > 0 || displayEvent.code)

  return (
    <button
      ref={buttonRef}
      onKeyDown={onKeyDown}
      onBlur={onBlur}
      onFocus={onFocus}
      tabIndex={0}
      className={`w-full h-8 px-3 rounded-md text-sm border shadow-sm text-left flex items-center transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring ${
        pending ? 'bg-background/50 border-foreground' : 'bg-background border-input'
      }`}
    >
      {hasShortcut ? (
        <KeyHint hotkey={displayEvent} />
      ) : (
        <span className="text-muted-foreground">Listening…</span>
      )}
    </button>
  )
}
