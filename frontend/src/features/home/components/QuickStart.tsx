import { quickStartItems } from '../../../content/site-content'

export function QuickStart() {
  return (
    <div className="mt-10 grid gap-4 lg:grid-cols-3">
      {quickStartItems.map((item, index) => (
        <article key={item.label} className="glass-panel rounded-[1.75rem] p-6">
          <div className="flex items-center justify-between gap-4">
            <span className="eyebrow">Step 0{index + 1}</span>
            <span className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--muted)]">CLI</span>
          </div>

          <h3 className="mt-5 text-2xl font-bold text-white">{item.label}</h3>
          <p className="mt-3 text-sm leading-6 text-[var(--muted)]">{item.description}</p>

          <div className="grid-panel mt-6 overflow-hidden rounded-[1.4rem] p-4">
            <div className="terminal-line font-['IBM_Plex_Mono'] text-[0.78rem] sm:text-[0.86rem]">
              <span className="terminal-prompt">$</span>
              <span className="terminal-copy break-all">{item.command}</span>
            </div>
          </div>
        </article>
      ))}
    </div>
  )
}