import { Rank, Tensor, env, setBackend, tensor } from '@tensorflow/tfjs-core'
import { GraphModel, loadGraphModel } from '@tensorflow/tfjs-converter'
import '@tensorflow/tfjs-backend-cpu'

export interface ModelResult {
  languageId: string
  confidence: number
}

let model: GraphModel | null = null
const MODEL_URL = '/model/model.json'

async function ensureModel() {
  if (model) return
  const tfEnv = env()
  tfEnv.set('IS_NODE', false)
  tfEnv.set('PROD', true)
  await setBackend('cpu')
  model = await loadGraphModel(MODEL_URL)
}

// Map model languageIds to Chroma lexer names. If an id is not mapped, it is skipped.
const MODEL_ID_TO_CHROMA: Record<string, string> = {
  ini: 'toml', // INI → TOML
  sql: 'sql',
  bat: 'sh', // Batch → Shell
  sh: 'sh',
  dockerfile: 'dockerfile',
  hs: 'hs',
  makefile: 'sh', // Makefile → Shell
  f90: 'matlab', // Fortran → Matlab
  asm: 'asm',
  yaml: 'yaml',
  java: 'java',
  rb: 'rb',
  kt: 'kt',
  ml: 'hs', // OCaml → Haskell
  groovy: 'java', // Groovy → Java
  scala: 'java', // Scala → Java
  coffee: 'js', // CoffeeScript → JavaScript
  py: 'py',
  pas: 'vb.net', // Pascal → VB.NET
  matlab: 'matlab',
  pm: 'pm', // Perl stays distinct (syntax is unique)
  lisp: 'clj', // Lisp → Clojure
  go: 'go',
  toml: 'toml',
  rs: 'rs',
  v: 'go', // V → Go
  lua: 'lua',
  clj: 'clj',
  swift: 'swift',
  r: 'matlab', // R → Matlab
  cmake: 'cmake',
  vba: 'vb.net',
  jl: 'matlab', // Julia → Matlab
  json: 'json',
  css: 'css',
  ps1: 'ps1',
  ex: 'rb', // Elixir → Ruby
  html: 'html',
  md: 'md',
  cs: 'c#',
  cpp: 'cpp',
  c: 'c',
  tex: 'md', // TeX → Markdown
  cbl: 'sql', // COBOL → SQL
  dart: 'dart',
  erl: 'rb', // Erlang → Ruby (via Elixir)
  prolog: 'prolog',
  xml: 'xml',
  ts: 'ts',
  mm: 'objective-c',
  php: 'php',
  js: 'js',
}

// Returns the first Chroma-compatible lexer name detected for the content, or null if none.
export async function detect(s: string): Promise<string> {
  await ensureModel()
  let content = s.replace(/\r\n/g, '\n')
  if (content.length < 100) {
    const repeats = Math.ceil(100 / content.length)
    content = Array(repeats).fill(content).join('\n')
  }

  const out = await model!.executeAsync(tensor([content]))
  const probsT: Tensor<Rank> = Array.isArray(out) ? out[0]! : out
  const langsT: Tensor<Rank> = Array.isArray(out) ? out[1]! : out
  const probs = probsT.dataSync() as Float32Array
  const langs: string[] = langsT.dataSync() as any
  const ranked: ModelResult[] = langs
    .map((languageId, i) => ({ languageId, confidence: probs[i] }))
    .sort((a, b) => b.confidence - a.confidence)

  console.log(ranked)
  for (const r of ranked) {
    const mapped = MODEL_ID_TO_CHROMA[r.languageId]
    if (mapped) return mapped
  }
  return 'plaintext'
}
