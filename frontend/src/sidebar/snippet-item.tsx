import { useState } from 'react'
import { Copy, Check } from 'lucide-react'
import type { SnippetPreview } from '../../bindings/zettl/pkg/models'

export function SnippetItem({
  snippet,
  isSelected,
  onClick,
  onCopy,
  fontSize,
}: {
  snippet: SnippetPreview
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
      className={`group relative w-full min-w-0 p-3 rounded-lg cursor-pointer border border-border/50 bg-card/50 hover:bg-accent/50 hover:border-accent-foreground/20 ${
        isSelected ? 'bg-card/90 ring-1 ring-primary/20 border-primary/30' : ''
      }`}
    >
      <div className="w-full overflow-hidden">
        <div
          className="font-mono leading-relaxed text-foreground/80 whitespace-pre w-[100ch] chroma"
          style={{ fontSize: `${fontSize}px` }}
          dangerouslySetInnerHTML={{ __html: snippet.html }}
        />
      </div>
      <button
        onClick={e => {
          e.stopPropagation()
          onCopy(snippet.content)
          setCopied(true)
          setTimeout(() => setCopied(false), 1000)
        }}
        className={`absolute top-2 right-2 p-1.5 rounded-md opacity-0 group-hover:opacity-100 bg-background/80 hover:bg-background border border-border/50 hover:border-border transition-all duration-200 ${
          copied
            ? 'bg-success/10 border-success/30 text-success dark:bg-success/20 dark:border-success/40 dark:text-success'
            : ''
        }`}
      >
        {copied ? <Check className="h-3 w-3" /> : <Copy className="h-3 w-3" />}
      </button>
    </div>
  )
}
