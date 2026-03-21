import type { ComponentType, SVGProps } from 'react'
import {
  BeakerIcon,
  BoltIcon,
  BugAntIcon,
  CircleStackIcon,
  CodeBracketSquareIcon,
  CommandLineIcon,
  CpuChipIcon,
  ExclamationTriangleIcon,
  ShieldCheckIcon,
} from '@heroicons/react/24/outline'

type IconType = ComponentType<SVGProps<SVGSVGElement>>

export type NavItem = {
  label: string
  href: string
}

export type DetectionFamily = {
  title: string
  description: string
  rules: string[]
  icon: IconType
}

export type PipelineStage = {
  name: string
  summary: string
  detail: string
  bullets: string[]
}

export type UseCase = {
  title: string
  description: string
  outcome: string
}

export type QuickStartItem = {
  label: string
  description: string
  command: string
}

export type Principle = {
  title: string
  description: string
}

export type Metric = {
  label: string
  value: string
  note: string
}

export const navigation: NavItem[] = [
  { label: 'Signals', href: '#features' },
  { label: 'Pipeline', href: '#pipeline' },
  { label: 'Use cases', href: '#use-cases' },
  { label: 'Quick start', href: '#quickstart' },
  { label: 'Principles', href: '#principles' },
]

export const trustPillars = [
  '30+ explainable heuristics across quality, security, performance, and tests',
  '.gitignore-aware repo discovery with generated-file and vendor filtering',
  'Compact text reports by default with JSON output for automation',
  'Syntax-tolerant parsing so partially broken trees still surface signal',
  'Repository-local symbol indexing for stronger local call checks',
  'Repeatable benchmarking built into the CLI surface',
]

export const terminalFlow = [
  {
    prompt: 'cargo run -- scan /path/to/go-repo',
    output: 'Compact findings and scan summary for fast review loops.',
  },
  {
    prompt: 'cargo run -- scan --json /path/to/go-repo',
    output: 'Structured output for CI, dashboards, and internal tooling.',
  },
  {
    prompt: 'cargo run -- bench --warmups 2 --repeats 5 /path/to/go-repo',
    output: 'Repeatable discovery, parse, index, heuristic, and total timings.',
  },
]

export const signalChips = [
  'missing_context',
  'hardcoded_secret',
  'goroutine_spawn_in_loop',
  'sql_string_concat',
  'test_without_assertion_signal',
  'hallucinated_local_call',
]

export const detectionFamilies: DetectionFamily[] = [
  {
    title: 'Naming and abstraction',
    description:
      'Spot generic or overdescriptive functions when the surrounding context stays weak or non-specific.',
    rules: ['generic_name', 'overlong_name', 'weak_typing'],
    icon: CodeBracketSquareIcon,
  },
  {
    title: 'Error handling',
    description:
      'Catch dropped errors, panic-first branches, and wrapping mistakes that make failures harder to reason about.',
    rules: ['dropped_error', 'panic_on_error', 'error_wrapping_misuse'],
    icon: ExclamationTriangleIcon,
  },
  {
    title: 'Security',
    description:
      'Flag weak crypto usage, direct secret literals, and string-built SQL patterns that deserve extra scrutiny.',
    rules: ['weak_crypto', 'hardcoded_secret', 'sql_string_concat'],
    icon: ShieldCheckIcon,
  },
  {
    title: 'Context and blocking',
    description:
      'Highlight missing context propagation, forgotten cancel paths, and blocking patterns that often age badly.',
    rules: ['missing_context', 'missing_cancel_call', 'busy_waiting'],
    icon: BoltIcon,
  },
  {
    title: 'Performance',
    description:
      'Surface repeated allocations, hot-path formatting, and full-payload reads before they turn into ambient latency.',
    rules: ['allocation_churn_in_loop', 'fmt_hot_path', 'full_dataset_load'],
    icon: CpuChipIcon,
  },
  {
    title: 'Concurrency',
    description:
      'Find coordination gaps in goroutines, lock pressure in loops, and shutdown paths that are missing or unclear.',
    rules: ['goroutine_without_coordination', 'goroutine_spawn_in_loop', 'mutex_in_loop'],
    icon: CircleStackIcon,
  },
  {
    title: 'Data access',
    description:
      'Call out query shapes that often correlate with N+1 patterns, wide reads, or indexing blind spots.',
    rules: ['n_plus_one_query', 'wide_select_query', 'likely_unindexed_query'],
    icon: CommandLineIcon,
  },
  {
    title: 'Tests and local hallucination',
    description:
      'Differentiate between tests that only gesture at safety and code that calls symbols the local repo cannot resolve.',
    rules: ['test_without_assertion_signal', 'placeholder_test_body', 'hallucinated_local_call'],
    icon: BeakerIcon,
  },
]

export const pipelineStages: PipelineStage[] = [
  {
    name: 'Discover',
    summary: 'Walk the target repository fast, with normal developer ignore rules respected by default.',
    detail:
      'deslop starts with file selection only. It skips vendor paths, filters non-Go inputs, and keeps discovery logic independent from analysis so later stages stay composable.',
    bullets: [
      '.gitignore-aware by default',
      'Skips vendor and common generated Go files',
      'Keeps discovery separate from parsing',
    ],
  },
  {
    name: 'Parse',
    summary: 'Use tree-sitter-go to extract syntax, symbols, imports, call sites, and function fingerprints.',
    detail:
      'The parser remains syntax tolerant. Even if a file is imperfect, deslop still tries to recover enough structure to keep signal flowing into the report.',
    bullets: [
      'Package names, imports, and declared symbols',
      'Call sites, loop markers, goroutine launches, and context clues',
      'Function-level fingerprints built for later heuristics',
    ],
  },
  {
    name: 'Index',
    summary: 'Build a lightweight repository-local symbol index keyed by package and directory context.',
    detail:
      'This stage is intentionally modest. It improves local selector and same-package checks without pretending to replace full Go type analysis.',
    bullets: [
      'Functions, methods, and declared symbol counts',
      'Package-plus-directory matching to reduce ambiguity',
      'Import context reused by hallucination heuristics',
    ],
  },
  {
    name: 'Heuristics',
    summary: 'Run explainable rule families that emit rule IDs, severity, messages, and evidence.',
    detail:
      'The heuristics layer is designed for human review rather than opaque scoring. Findings stay readable and conservative where deeper semantic proof does not exist yet.',
    bullets: [
      'Compact text output by default, details opt in',
      'JSON output for pipeline integration',
      'Readable evidence payloads instead of black-box scores',
    ],
  },
]

export const useCases: UseCase[] = [
  {
    title: 'Review AI-assisted pull requests',
    description:
      'Use deslop as a second pass when code looks plausible but lacks the domain texture, shutdown discipline, or test intent you would expect from a mature change.',
    outcome: 'Shortens review time by surfacing the suspicious shapes first.',
  },
  {
    title: 'Harden platform and infra code',
    description:
      'Context propagation, goroutine coordination, and loop-driven locking mistakes show up early when teams move quickly across handlers, jobs, and background workers.',
    outcome: 'Pushes risk discovery earlier than post-incident analysis.',
  },
  {
    title: 'Run targeted security sweeps',
    description:
      'Weak crypto, secret literals, and string-built query paths are called out as explainable findings that can feed human security review.',
    outcome: 'Adds a narrow security lens without pretending to be a full audit suite.',
  },
  {
    title: 'Feed internal automation',
    description:
      'The CLI surface already supports JSON output and benchmarking, so the tool can sit in CI or local tooling without a database or background service.',
    outcome: 'Keeps adoption cheap for teams that prefer simple workflows.',
  },
]

export const quickStartItems: QuickStartItem[] = [
  {
    label: 'Build the binary',
    description: 'Create a native release binary for your current platform before scanning real repositories.',
    command: 'cargo build --release',
  },
  {
    label: 'Scan a repository',
    description: 'Run the default scan for a compact summary plus the main finding set.',
    command: 'cargo run -- scan /path/to/go-repo',
  },
  {
    label: 'Integrate with tooling',
    description: 'Use structured JSON when you need machine-readable output inside scripts or CI.',
    command: 'cargo run -- scan --json /path/to/go-repo',
  },
]

export const principles: Principle[] = [
  {
    title: 'Heuristics, not proof',
    description:
      'deslop is intentionally explicit about what it can and cannot prove. It surfaces suspicious patterns quickly and leaves the final judgment to engineers.',
  },
  {
    title: 'Repository-local context first',
    description:
      'The current index and hallucination checks stay local to the scanned repository. That keeps the tool fast and honest about its scope.',
  },
  {
    title: 'Readable evidence over black-box scoring',
    description:
      'Each finding is meant to be legible in a code review workflow: rule ID, message, severity, and the evidence needed to validate it.',
  },
]

export const metrics: Metric[] = [
  {
    label: 'Mean benchmark time',
    value: '180.80 ms',
    note: 'Preferred local baseline documented for gopdfsuit.',
  },
  {
    label: 'Benchmark repository scale',
    value: '89 files / 702 functions',
    note: 'Measured as a full-repository static analysis pass.',
  },
  {
    label: 'CLI surface',
    value: 'scan, scan --json, bench',
    note: 'Compact output by default, details available when needed.',
  },
]

export const footerLinks: NavItem[] = [
  { label: 'Back to top', href: '#top' },
  { label: 'Detection families', href: '#features' },
  { label: 'Pipeline', href: '#pipeline' },
  { label: 'Quick start', href: '#quickstart' },
]

export const footerSources = [
  'README command surface and scan modes',
  'Features and detections guide for rule families and philosophy',
  'Implementation guide for pipeline stages and benchmark baseline',
]

export const noteCards = [
  {
    title: 'Explainable output',
    description: 'Readable findings with rule IDs and evidence help reviewers decide what matters.',
    icon: BugAntIcon,
  },
  {
    title: 'Go-specific scope',
    description: 'The current implementation is tuned for Go repositories rather than generic multi-language analysis.',
    icon: CodeBracketSquareIcon,
  },
  {
    title: 'Open extension path',
    description: 'The architecture is already split into stages so later analysis can reuse stable intermediate results.',
    icon: CircleStackIcon,
  },
]