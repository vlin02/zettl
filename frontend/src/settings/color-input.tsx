import { Input } from '../components/ui/input'

export function ColorInput({
  value,
  onChange,
}: {
  value: string
  onChange: (color: string) => void
}) {
  return (
    <div className="flex items-center gap-2">
      <input
        type="color"
        value={value}
        onChange={e => onChange(e.target.value)}
        className="h-8 w-16 rounded cursor-pointer"
      />
      <Input
        value={value}
        onChange={e => onChange(e.target.value)}
        className="h-8"
        placeholder="#ffffff"
      />
    </div>
  )
}
