import { forwardRef, useCallback } from 'react'
import { ChevronUp, ChevronDown } from 'lucide-react'
import { Input } from './input'
import { Button } from './button'

interface NumberInputProps {
  value: number
  onChange: (v: number) => void
  min?: number
  max?: number
  step?: number
  className?: string
  suffix?: string
}

export const NumberInput = forwardRef<HTMLInputElement, NumberInputProps>(
  ({ value, onChange, min = -Infinity, max = Infinity, step = 1, className = '', suffix }, ref) => {
    const clamp = useCallback((v: number) => Math.min(Math.max(v, min), max), [min, max])
    const set = (v: number) => {
      const c = clamp(v)
      if (c !== value) onChange(c)
    }
    return (
      <div className={className + ' relative flex items-center'}>
        <Input
          ref={ref}
          type="number"
          value={value}
            min={Number.isFinite(min) ? min : undefined}
            max={Number.isFinite(max) ? max : undefined}
          onChange={e => set(parseInt(e.target.value, 10) || min || 0)}
          onKeyDown={e => {
            if (e.key === 'ArrowUp') {
              e.preventDefault()
              set(value + step)
            } else if (e.key === 'ArrowDown') {
              e.preventDefault()
              set(value - step)
            }
          }}
          className="pr-14 [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
        />
        {suffix && (
          <span className="absolute right-8 top-1/2 -translate-y-1/2 text-xs text-muted-foreground select-none">
            {suffix}
          </span>
        )}
        <div className="absolute right-0 top-0 bottom-0 flex flex-col justify-center py-1">
          <Button
            type="button"
            variant="ghost"
            className="h-4 px-1 rounded-none"
            onClick={() => set(value + step)}
            disabled={value >= max}
          >
            <ChevronUp className="h-3 w-3" />
          </Button>
          <Button
            type="button"
            variant="ghost"
            className="h-4 px-1 rounded-none"
            onClick={() => set(value - step)}
            disabled={value <= min}
          >
            <ChevronDown className="h-3 w-3" />
          </Button>
        </div>
      </div>
    )
  }
)
NumberInput.displayName = 'NumberInput'
