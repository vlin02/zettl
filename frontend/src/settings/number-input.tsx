import { forwardRef } from 'react'
import { ChevronUp, ChevronDown } from 'lucide-react'
import { Input } from '../components/ui/input'
import { Button } from '../components/ui/button'

interface NumberInputProps {
  value: number
  onChange: (v: number) => void
  min?: number
  max?: number
  step?: number
  suffix?: string
}

export const NumberInput = forwardRef<HTMLInputElement, NumberInputProps>(
  ({ value, onChange, min, max, step = 1, suffix }, ref) => {
    const clamp = (v: number) => Math.min(Math.max(v, min ?? -Infinity), max ?? Infinity)
    return (
      <div className="flex items-center w-full">
        <div className="relative flex-1">
          <Input
            ref={ref}
            type="number"
            value={value}
            min={min}
            max={max}
            step={step}
            onChange={e => onChange(parseFloat(e.target.value) || 0)}
            className={`h-8 rounded-r-none ${suffix ? 'pr-8' : ''} [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none`}
          />
          {suffix && (
            <span className="absolute right-2 top-1/2 -translate-y-1/2 text-sm text-muted-foreground pointer-events-none">
              {suffix}
            </span>
          )}
        </div>

        <div className="flex flex-col">
          <Button
            type="button"
            className="px-1 py-0 h-4 rounded-l-none rounded-br-none border-l-0 border-b-[0.5px]"
            variant="outline"
            onClick={() => onChange(clamp(value + step))}
          >
            <ChevronUp className="!h-3" />
          </Button>
          <Button
            type="button"
            className="px-1 py-0 h-4 rounded-l-none rounded-tr-none border-l-0 border-b-[0.5px]"
            variant="outline"
            onClick={() => onChange(clamp(value - step))}
          >
            <ChevronDown className="!h-3" />
          </Button>
        </div>
      </div>
    )
  },
)
NumberInput.displayName = 'NumberInput'
