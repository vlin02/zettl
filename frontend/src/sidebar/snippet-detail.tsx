import { getTypeIcon, Clock } from "./clipboard-icons";
import { X, Copy } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import * as Zettl from "../../bindings/zettl/service";
import {
  SnippetPreview,
  SnippetDetail as SnippetDetailType,
} from "../../bindings/zettl/pkg/models";

export function SnippetDetail({
  snippet,
  onCopy,
  onClose,
  fontSize,
}: {
  snippet: SnippetPreview;
  onCopy: (text: string) => void;
  onClose: () => void;
  fontSize: number;
}) {
  const [detail, setDetail] = useState<SnippetDetailType | null>(null);
  const timer = useRef<number | null>(null);

  useEffect(() => {
    const id = setTimeout(() => {
      Zettl.GetSnippetDetail(snippet.id).then(setDetail);
    }, 50);

    return () => clearTimeout(id);
  }, [snippet.id]);

  useEffect(() => {
    if (!detail) return;
    if (timer.current) window.clearTimeout(timer.current);
    return () => {
      if (timer.current) window.clearTimeout(timer.current);
    };
  }, [detail?.id, detail?.html]);

  if (!detail) return null;

  const type = detail.language ? "code" : "text";
  const ts = new Date(detail.copied_at * 1000);

  return (
    <div className="w-full h-full flex border-l border-border/30 backdrop-blur-sm">
      <div className="w-full p-6 flex flex-col h-full">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-2">
            {getTypeIcon(type, detail.language)}
            <span className="text-sm font-semibold">{detail.language}</span>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={onClose}
              title="Close"
              className="h-8 w-8 rounded border border-border/40 hover:bg-accent/30 flex items-center justify-center"
            >
              <X className="h-4 w-4" />
            </button>
            <button
              onClick={() => onCopy(detail.content)}
              title="Copy"
              className="h-8 w-8 rounded bg-primary text-primary-foreground hover:bg-primary/90 flex items-center justify-center"
            >
              <Copy className="h-4 w-4" />
            </button>
          </div>
        </div>
        <div className="text-xs text-muted-foreground mb-4 flex items-center gap-2">
          <Clock className="h-3 w-3" />
          {ts.toLocaleString()}
        </div>
        <div className="flex-1 min-h-0 overflow-y-auto">
          <div
            className={
              "p-4 rounded-lg border border-border/30 chroma bg-muted/30 dark:bg-card/60"
            }
          >
            <div className="w-full overflow-x-auto">
              <div
                className="font-mono leading-relaxed text-foreground/80 whitespace-pre w-[120ch]"
                style={{ fontSize: `${fontSize}px` }}
                dangerouslySetInnerHTML={{ __html: detail.html }}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
