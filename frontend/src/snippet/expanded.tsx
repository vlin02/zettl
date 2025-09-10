import { getTypeIcon, Clock } from './language'
import { X, Copy } from 'lucide-react'
import { Button } from '../components/ui/button'
import { useEffect, useRef, useState } from 'react'
import { SnippetPreview, SnippetDetail } from '../../bindings/zettl/pkg/models'
import { GetSnippetDetail } from '../../bindings/zettl/service'

export function ExpandedView({
  snippet,
  onCopy,
  onClose,
  fontSize,
}: {
  snippet: SnippetPreview
  onCopy: (text: string) => void
  onClose: () => void
  fontSize: number
}) {
  const [detail, setDetail] = useState<SnippetDetail | null>(null)
  const timer = useRef<number | null>(null)

  useEffect(() => {
    const id = setTimeout(() => {
      GetSnippetDetail(snippet.id).then(setDetail)
    }, 50)

    return () => clearTimeout(id)
  }, [snippet.id])

  useEffect(() => {
    if (!detail) return
    if (timer.current) window.clearTimeout(timer.current)
    return () => {
      if (timer.current) window.clearTimeout(timer.current)
    }
  }, [detail?.id, detail?.html])

  if (!detail) return null

  const ts = new Date(detail.copied_at * 1000)

  return (
    <div className="h-full border-l border-border/30">
      <div className="p-6 flex flex-col h-full">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-2">
            <div className="text-xs text-muted-foreground flex items-center gap-2">
              <Clock className="h-3 w-3" />
              {ts.toLocaleString()}
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Button
              onClick={() => onCopy(detail.content)}
              title="Copy"
              size="icon"
              className="h-8 w-8"
            >
              <Copy className="h-4 w-4" />
            </Button>
          </div>
        </div>
        <div className="flex-1 min-h-0 overflow-y-auto">
          <div
            className={'p-4 rounded-lg border border-border/30 chroma bg-muted/30 dark:bg-card/60'}
          >
            <div className="overflow-x-auto">
              <div
                className="font-mono leading-relaxed text-foreground/80 whitespace-pre"
                style={{ fontSize: `${fontSize}px` }}
                dangerouslySetInnerHTML={{ __html: detail.html }}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
