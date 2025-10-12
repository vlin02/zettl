import { useEffect, useRef, useState } from 'react'
import { Snippet, Shortcut } from '../../bindings/zettl/pkg'
import { GetSnippetDetail } from '../../bindings/zettl/service'
import { KeyHint } from '../shortcut/hint'
import { Clock } from 'lucide-react'

export function ExpandedView({ snippet, fontSize }: { snippet: Snippet; fontSize: number }) {
  const [detail, setDetail] = useState<Snippet | null>(null)
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
  }, [detail?.id])

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
          <div className="flex items-center gap-3 text-xs text-muted-foreground">
            <div className="flex items-center gap-1">
              <KeyHint hotkey={new Shortcut({ modifiers: [], code: 'Enter' })} />
              <span>copy</span>
            </div>
            <div className="h-3 w-px bg-border/50" />
            <div className="flex items-center gap-1">
              <KeyHint hotkey={new Shortcut({ modifiers: ['Meta'], code: 'Enter' })} />
              <span>copy + paste</span>
            </div>
          </div>
        </div>
        <div className="flex-1 min-h-0 overflow-y-auto">
          <div
            className={'p-4 rounded-lg border border-border/30 chroma bg-muted/30 dark:bg-card/60'}
          >
            <div className="overflow-x-auto">
              {detail.html ? (
                <div
                  className="font-mono leading-relaxed text-foreground/80 whitespace-pre"
                  style={{ fontSize: `${fontSize}px` }}
                  dangerouslySetInnerHTML={{ __html: detail.html }}
                />
              ) : (
                <div
                  className="font-mono leading-relaxed text-foreground/80 whitespace-pre"
                  style={{ fontSize: `${fontSize}px` }}
                >
                  {detail.content}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
