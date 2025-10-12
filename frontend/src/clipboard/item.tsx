import { useState } from 'react'
import { Copy, Check } from 'lucide-react'
import type { Snippet } from '../../bindings/zettl/pkg'

export function SnippetItem({
  snippet,
  isSelected,
  onClick,
  onCopy,
  fontSize,
}: {
  snippet: Snippet
  isSelected: boolean
  onClick: () => void
  onCopy: (text: string) => void
  fontSize: number
}) {
  const [copied, setCopied] = useState(false)

  return (
    <div
      id={`snip-${snippet.id}`}
      onClick={onClick}
      className={`group relative p-3 rounded-lg cursor-pointer border border-border/50 ${
        isSelected ? 'bg-accent' : 'bg-card/60 hover:bg-accent/30'
      }`}
    >
      <div className="overflow-hidden">
        {snippet.html ? (
          <div
            className="font-mono leading-relaxed text-foreground/80 whitespace-pre w-[100ch] chroma"
            style={{ fontSize: `${fontSize}px` }}
            dangerouslySetInnerHTML={{ __html: snippet.html }}
          />
        ) : (
          <div
            className="font-mono leading-relaxed text-foreground/80 whitespace-pre w-[100ch]"
            style={{ fontSize: `${fontSize}px` }}
          >
            {snippet.content.split('\n').slice(0, 5).join('\n')}
          </div>
        )}
      </div>
      <button
        onClick={e => {
          e.stopPropagation()
          onCopy(snippet.content)
          setCopied(true)
          setTimeout(() => setCopied(false), 1000)
        }}
        className={`absolute top-2 right-2 p-1.5 rounded-md opacity-0 group-hover:opacity-100 bg-background/80 hover:bg-background/90 border border-border/50 transition-colors ${
          copied ? 'bg-success/15 border-success/40 text-success' : ''
        }`}
      >
        {copied ? <Check className="h-3 w-3" /> : <Copy className="h-3 w-3" />}
      </button>
    </div>
  )
}
