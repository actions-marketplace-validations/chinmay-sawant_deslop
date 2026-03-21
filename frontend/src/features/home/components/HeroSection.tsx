import { ArrowRightIcon } from '@heroicons/react/24/outline'

import { signalChips, terminalFlow, trustPillars } from '../../../content/site-content'
import { Container } from '../../../shared/ui/Container'

export function HeroSection() {
  return (
    <section id="top" className="section-anchor relative pt-14 pb-18 sm:pt-18 sm:pb-24 lg:pt-24 lg:pb-32">
      <div className="orb left-[-12rem] top-16 h-72 w-72" aria-hidden="true" />
      <div className="orb orb-delay right-[-10rem] top-32 h-64 w-64" aria-hidden="true" />

      <Container className="relative grid items-start gap-10 lg:grid-cols-[1.08fr_0.92fr] lg:gap-12">
        <div>
          <span className="eyebrow">Low-context code leaves a shape</span>
          <h1 className="mt-6 max-w-4xl text-5xl leading-[0.96] font-bold tracking-[-0.06em] text-white sm:text-6xl lg:text-[4.75rem]">
            Catch the AI-flavored Go patterns before they become platform debt.
          </h1>
          <p className="mt-6 max-w-2xl text-lg text-[var(--muted)] sm:text-xl">
            deslop scans Go repositories for the structural signals often associated with low-context or AI-assisted code:
            generic naming, weak typing, shaky error handling, concurrency risk, performance smells, hallucinated local calls,
            and test bodies that only look complete.
          </p>

          <div className="mt-8 flex flex-col gap-3 sm:flex-row">
            <a href="#quickstart" className="button-primary">
              Run the first scan
              <ArrowRightIcon className="h-4 w-4" aria-hidden="true" />
            </a>
            <a href="#pipeline" className="button-secondary">
              See how it works
            </a>
          </div>

          <div className="mt-10 grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            {trustPillars.slice(0, 3).map((pillar) => (
              <div key={pillar} className="stat-pill rounded-2xl px-4 py-4 text-sm leading-6">
                {pillar}
              </div>
            ))}
          </div>
        </div>

        <div className="glass-panel rounded-[2rem] p-4 sm:p-6 lg:p-7">
          <div className="flex items-center justify-between gap-4 border-b border-white/8 pb-4">
            <div>
              <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--accent)]">Representative CLI flow</p>
              <p className="mt-2 text-sm text-[var(--muted)]">Use compact text output locally, then switch to JSON when the workflow needs machine-readable detail.</p>
            </div>
            <div className="flex gap-2" aria-hidden="true">
              <span className="h-3 w-3 rounded-full bg-[#ff7a7a]" />
              <span className="h-3 w-3 rounded-full bg-[#facc15]" />
              <span className="h-3 w-3 rounded-full bg-[var(--accent)]" />
            </div>
          </div>

          <div className="grid-panel mt-5 rounded-[1.6rem] p-5 sm:p-6">
            <div className="space-y-4 text-sm sm:text-[0.95rem]">
              {terminalFlow.map((item) => (
                <div key={item.prompt} className="space-y-2 rounded-2xl border border-white/6 bg-black/10 p-4">
                  <div className="terminal-line font-['IBM_Plex_Mono'] text-[0.82rem] sm:text-[0.9rem]">
                    <span className="terminal-prompt">$</span>
                    <span className="terminal-copy break-all">{item.prompt}</span>
                  </div>
                  <p className="pl-6 text-[var(--muted)]">{item.output}</p>
                </div>
              ))}
            </div>

            <div className="mt-6 border-t border-white/8 pt-5">
              <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--muted)]">Representative rule IDs</p>
              <div className="mt-3 flex flex-wrap gap-2">
                {signalChips.map((chip) => (
                  <span key={chip} className="rounded-full border border-[var(--border-strong)] bg-[var(--accent-soft)] px-3 py-1.5 font-['IBM_Plex_Mono'] text-[0.75rem] text-[var(--code)]">
                    {chip}
                  </span>
                ))}
              </div>
            </div>
          </div>
        </div>
      </Container>
    </section>
  )
}