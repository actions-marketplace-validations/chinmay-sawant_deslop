import { ArrowRightIcon } from '@heroicons/react/24/outline'

import { terminalFlow, trustPillars } from '../../../content/site-content'
import { Container } from '../../../shared/ui/Container'

export function HeroSection() {
  return (
    <section id="top" className="section-anchor relative pt-18 pb-22 sm:pt-24 sm:pb-28 lg:pt-32 lg:pb-36">
      <Container className="relative grid items-start gap-14 lg:grid-cols-[1.15fr_0.85fr] lg:gap-20">
        <div>
          <span className="eyebrow">Static analysis, without the noise</span>
          <h1 className="mt-7 max-w-5xl text-5xl leading-[0.92] font-bold tracking-[-0.065em] text-white sm:text-6xl lg:text-[5.5rem]">
            A calmer way to review low-context code.
          </h1>
          <p className="mt-7 max-w-3xl text-lg leading-8 text-[var(--muted)] sm:text-xl">
            deslop helps teams surface the patterns that often appear when code is generated quickly and reviewed too late: vague naming,
            brittle error handling, security smells, thin tests, and local context that does not quite add up. The current implementation starts with Go repositories,
            but the product story does not need to feel boxed in by that scope.
          </p>

          <div className="mt-10 flex flex-col gap-3 sm:flex-row">
            <a href="#quickstart" className="button-primary">
              View quick start
              <ArrowRightIcon className="h-4 w-4" aria-hidden="true" />
            </a>
            <a href="#pipeline" className="button-secondary">
              See the pipeline
            </a>
          </div>

          <div className="mt-12 grid max-w-4xl gap-4 md:grid-cols-3">
            {trustPillars.map((pillar) => (
              <div key={pillar} className="rounded-[1.6rem] border border-white/8 bg-white/2 px-5 py-5 text-sm leading-7 text-[#e5e5e5]">
                {pillar}
              </div>
            ))}
          </div>
        </div>

        <div className="glass-panel rounded-[2rem] p-5 sm:p-7 lg:p-8">
          <div className="flex items-center justify-between gap-4 border-b border-white/8 pb-4">
            <div>
              <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-white">Current implementation</p>
              <p className="mt-2 text-sm leading-7 text-[var(--muted)]">A minimal CLI surface with compact output by default, JSON when automation needs structure, and benchmark mode for repeatable measurements.</p>
            </div>
            <div className="flex gap-2" aria-hidden="true">
              <span className="h-3 w-3 rounded-full bg-[#ffffff]" />
              <span className="h-3 w-3 rounded-full bg-[#a3a3a3]" />
              <span className="h-3 w-3 rounded-full bg-[#525252]" />
            </div>
          </div>

          <div className="grid-panel mt-6 rounded-[1.75rem] p-6 sm:p-7">
            <div className="space-y-4 text-sm sm:text-[0.95rem]">
              {terminalFlow.map((item) => (
                <div key={item.prompt} className="space-y-2 rounded-2xl border border-white/6 bg-black/15 p-4 sm:p-5">
                  <div className="terminal-line font-['IBM_Plex_Mono'] text-[0.82rem] sm:text-[0.9rem]">
                    <span className="terminal-prompt">$</span>
                    <span className="terminal-copy break-all">{item.prompt}</span>
                  </div>
                  <p className="pl-6 leading-7 text-[var(--muted)]">{item.output}</p>
                </div>
              ))}
            </div>

            <div className="mt-7 border-t border-white/8 pt-6">
              <div className="grid gap-4 sm:grid-cols-3">
                <div>
                  <p className="font-['IBM_Plex_Mono'] text-[0.7rem] uppercase tracking-[0.18em] text-[var(--muted)]">Scope</p>
                  <p className="mt-2 text-sm leading-7 text-[#e7e7e7]">Today the analyzer targets Go repositories and their local project context.</p>
                </div>
                <div>
                  <p className="font-['IBM_Plex_Mono'] text-[0.7rem] uppercase tracking-[0.18em] text-[var(--muted)]">Output</p>
                  <p className="mt-2 text-sm leading-7 text-[#e7e7e7]">Readable findings first, detailed output only when you ask for it.</p>
                </div>
                <div>
                  <p className="font-['IBM_Plex_Mono'] text-[0.7rem] uppercase tracking-[0.18em] text-[var(--muted)]">Intent</p>
                  <p className="mt-2 text-sm leading-7 text-[#e7e7e7]">More useful review signals, not a fake promise of perfect proof.</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </Container>
    </section>
  )
}