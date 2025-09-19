import { useEffect, useState } from 'react'
import { X } from 'lucide-react'
import { Button } from '../components/ui/button'
import {
  ListStyles,
  SetSyntaxStyle,
  SetRetentionDays,
  SetToggleHotkey,
  SetFontSize,
} from '../../bindings/zettl/service'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../components/ui/select'
import { NumberInput } from './number-input'
import { ShortcutInput } from '../shortcut/input'
import type { UISettings } from '../../bindings/zettl/pkg/models'

export function SettingsPanel({
  onClose,
  settings,
  onRefetch,
}: {
  onClose: () => void
  settings: UISettings
  onRefetch: () => void
}) {
  const [styles, setStyles] = useState<string[]>([])
  useEffect(() => {
    ListStyles().then(setStyles)
  }, [])

  return (
    <div className="w-full h-full flex flex-col">
      <div className="p-4 flex items-center justify-between">
        <h2 className="text-lg font-semibold">Settings</h2>
        <Button
          className="h-8 w-8"
          size="icon"
          variant="secondary"
          onClick={onClose}
          title="Close settings"
        >
          <X className="h-4 w-4 text-muted-foreground" />
        </Button>
      </div>
      <div className="flex-1 overflow-y-auto">
        <div className="p-4 space-y-4">
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium w-40">Theme</span>
            <Select
              value={settings.style.name}
              onValueChange={async v => {
                await SetSyntaxStyle(v)
                onRefetch()
              }}
            >
              <SelectTrigger className="w-56 h-8">
                <SelectValue />
              </SelectTrigger>
              <SelectContent className="max-h-96 overflow-y-auto">
                {styles.map(t => (
                  <SelectItem key={t} value={t}>
                    {t}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium w-40">Font Size</span>
            <div className="w-56 h-8">
              <NumberInput
                value={settings.font_size}
                min={8}
                max={32}
                step={1}
                suffix="px"
                onChange={async size => {
                  await SetFontSize(size)
                  onRefetch()
                }}
              />
            </div>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium w-40">Data Retention</span>
            <Select
              value={String(settings.retention_days)}
              onValueChange={async (v: string) => {
                const n = parseInt(v, 10)
                await SetRetentionDays(n)
                onRefetch()
              }}
            >
              <SelectTrigger className="w-56 h-8">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="1">1 day</SelectItem>
                <SelectItem value="7">1 week</SelectItem>
                <SelectItem value="30">1 month</SelectItem>
                <SelectItem value="90">3 months</SelectItem>
                <SelectItem value="365">1 year</SelectItem>
                <SelectItem value="0">Forever</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium w-40">Activate Zettl</span>
            <div className="w-56 h-8">
              <ShortcutInput
                event={settings.toggle_hotkey}
                onSubmit={async ev => {
                  await SetToggleHotkey(ev)
                  onRefetch()
                }}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
