import { useEffect, useState } from 'react'
import { X, Trash2, Palette, Keyboard } from 'lucide-react'
import { Button } from '../components/ui/button'
import { ListStyles, SetSyntaxStyle, SetRetentionDays, SetToggleHotkey, SetBgColors, SetFontSize } from '../../bindings/zettl/service'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../components/ui/select'
import { Input } from '../components/ui/input'
import { ShortcutInput } from './shortcut-input'
import { ColorInput } from './color-input'
import type { UISettings } from '../../bindings/zettl/pkg/models'

export function SettingsPanel({
  isOpen,
  onClose,
  settings,
  onRefetch,
}: {
  isOpen: boolean
  onClose: () => void
  settings: UISettings
  onRefetch: () => void
}) {
  const [styles, setStyles] = useState<string[]>([])
  useEffect(() => {
    if (isOpen) ListStyles().then(setStyles)
  }, [isOpen])

  if (!isOpen) return null

  return (
    <div className="w-full h-full flex flex-col">
      <div className="p-4 border-b border-border/30 flex items-center justify-between">
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
        <div className="p-4 space-y-6">
          <div className="space-y-4">
            <div className="flex items-center gap-2">
              <Trash2 className="h-4 w-4 text-muted-foreground" />
              <h3 className="font-medium">Storage</h3>
            </div>
            <div className="space-y-3 pl-6">
              <div className="space-y-2">
                <label className="text-sm text-muted-foreground">Delete snippets after</label>
                <Select
                  value={String(settings.retention_days)}
                  onValueChange={async (v: string) => {
                    const n = parseInt(v, 10)
                    await SetRetentionDays(n)
                    onRefetch()
                  }}
                >
                  <SelectTrigger className="h-8">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="1">1 day</SelectItem>
                    <SelectItem value="3">3 days</SelectItem>
                    <SelectItem value="7">1 week</SelectItem>
                    <SelectItem value="14">2 weeks</SelectItem>
                    <SelectItem value="30">1 month</SelectItem>
                    <SelectItem value="90">3 months</SelectItem>
                    <SelectItem value="365">1 year</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
          </div>
          <div className="border-t border-border/30" />
          <div className="space-y-4">
            <div className="flex items-center gap-2">
              <Keyboard className="h-4 w-4 text-muted-foreground" />
              <h3 className="font-medium">Shortcuts</h3>
            </div>
            <div className="space-y-3 pl-6">
              <div className="space-y-2">
                <label className="text-sm text-muted-foreground">Toggle sidebar</label>
                <div>
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
          <div className="border-t border-border/30" />
          <div className="space-y-4">
            <div className="flex items-center gap-2">
              <Palette className="h-4 w-4 text-muted-foreground" />
              <h3 className="font-medium">Appearance</h3>
            </div>
            <div className="space-y-3 pl-6">
              <div className="space-y-2">
                <label className="text-sm text-muted-foreground">Style</label>
                <Select
                  value={settings.style.name}
                  onValueChange={async v => {
                    await SetSyntaxStyle(v)
                    onRefetch()
                  }}
                >
                  <SelectTrigger className="h-8">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {styles.map(t => (
                      <SelectItem key={t} value={t}>
                        {t}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-2">
                <label className="text-sm text-muted-foreground">Background</label>
                <ColorInput
                  value={settings.theme === 'light' ? settings.light_bg_color : settings.dark_bg_color}
                  onChange={async color => {
                    await SetBgColors(
                      settings.theme === 'light' ? color : settings.light_bg_color,
                      settings.theme === 'dark' ? color : settings.dark_bg_color
                    )
                    onRefetch()
                  }}
                />
              </div>
              <div className="space-y-2">
                <label className="text-sm text-muted-foreground">Font size (px)</label>
                <Input
                  type="number"
                  min="8"
                  max="32"
                  value={settings.font_size}
                  onChange={async e => {
                    const size = parseInt(e.target.value, 10)
                    if (size >= 8 && size <= 32) {
                      await SetFontSize(size)
                      onRefetch()
                    }
                  }}
                  className="h-8"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
