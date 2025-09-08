import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'

function App() {
  const [switchOn, setSwitchOn] = useState(false)
  const [value, setValue] = useState('')
  const [choice, setChoice] = useState<string | undefined>()

  return (
    <div className="min-h-screen bg-background text-foreground p-8 space-y-8">
      <header className="space-y-1">
        <h1 className="text-2xl font-semibold tracking-tight">Component demo</h1>
        <p className="text-sm text-muted-foreground">shadcn/ui components wired up inside Wails + React.</p>
      </header>
      <section className="space-y-6">
        <div className="space-y-2">
          <h2 className="text-lg font-medium">Buttons</h2>
          <div className="flex flex-wrap gap-2">
            <Button onClick={() => console.log("wef")}>Default</Button>
            <Button variant="outline">Outline</Button>
            <Button variant="ghost">Ghost</Button>
          </div>
        </div>
        <div className="space-y-2">
          <h2 className="text-lg font-medium">Input</h2>
          <div className="flex gap-2 items-center">
            <Input className="w-64" placeholder="Type something" value={value} onChange={e=>setValue(e.target.value)} />
            <span className="text-sm text-muted-foreground truncate max-w-xs">Value: {value || '—'}</span>
          </div>
        </div>
        <div className="space-y-2">
          <h2 className="text-lg font-medium">Switch</h2>
            <div className="flex items-center gap-3">
              <Switch checked={switchOn} onCheckedChange={setSwitchOn} />
              <span className="text-sm text-muted-foreground">{switchOn ? 'On' : 'Off'}</span>
            </div>
        </div>
        <div className="space-y-2">
          <h2 className="text-lg font-medium">Select</h2>
          <div className="w-48">
            <Select value={choice} onValueChange={setChoice}>
              <SelectTrigger>
                <SelectValue placeholder="Choose" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="one">One</SelectItem>
                <SelectItem value="two">Two</SelectItem>
                <SelectItem value="three">Three</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <p className="text-sm text-muted-foreground">Selected: {choice || '—'}</p>
        </div>
      </section>
    </div>
  )
}

export default App
