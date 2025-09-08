import {
  Search,
  Clock,
  Code,
  FileText,
  Image,
  Link,
  Braces,
  Database,
  Globe,
  Palette,
  Terminal,
  FileCode,
  Hash,
  Zap,
  Box,
  Settings,
  Layers,
} from 'lucide-react'

export { Search, Clock }

export const getTypeIcon = (type: string, language?: string) => {
  if (language) {
    switch (language.toLowerCase()) {
      case 'javascript':
      case 'js':
        return <Zap className="h-3 w-3" />
      case 'typescript':
      case 'ts':
        return <Box className="h-3 w-3" />
      case 'python':
      case 'py':
        return <Hash className="h-3 w-3" />
      case 'json':
        return <Braces className="h-3 w-3" />
      case 'sql':
        return <Database className="h-3 w-3" />
      case 'html':
        return <Globe className="h-3 w-3" />
      case 'css':
      case 'scss':
      case 'sass':
        return <Palette className="h-3 w-3" />
      case 'bash':
      case 'shell':
      case 'sh':
        return <Terminal className="h-3 w-3" />
      case 'markdown':
      case 'md':
        return <FileCode className="h-3 w-3" />
      case 'yaml':
      case 'yml':
        return <Settings className="h-3 w-3" />
      case 'xml':
        return <Layers className="h-3 w-3" />
      default:
        return <Code className="h-3 w-3" />
    }
  }
  if (type === 'url') return <Link className="h-3 w-3" />
  if (type === 'image') return <Image className="h-3 w-3" />
  if (type === 'code') return <Code className="h-3 w-3" />
  return <FileText className="h-3 w-3" />
}

export const formatTimeAgo = (date: Date) => {
  const now = new Date()
  const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / 60000)
  if (diffInMinutes < 1) return 'Just now'
  if (diffInMinutes < 60) return `${diffInMinutes}m ago`
  const diffInHours = Math.floor(diffInMinutes / 60)
  if (diffInHours < 24) return `${diffInHours}h ago`
  const diffInDays = Math.floor(diffInHours / 24)
  return `${diffInDays}d ago`
}
