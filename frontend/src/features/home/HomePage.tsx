import { ArrowRightIcon } from '@heroicons/react/24/outline'

import {
  footerLinks,
  footerSources,
  metrics,
  noteCards,
  principles,
  trustPillars,
  useCases,
} from '../../content/site-content'
import { Container } from '../../shared/ui/Container'
import { SectionIntro } from '../../shared/ui/SectionIntro'
import { FeatureGrid } from './components/FeatureGrid'
import { Header } from './components/Header'
import { HeroSection } from './components/HeroSection'
import { PipelineTabs } from './components/PipelineTabs'
import { QuickStart } from './components/QuickStart'

export function HomePage() {
  return (
    <div className="relative">
      <Header />

      <main>
        <HeroSection />

        <section className="section-anchor pb-8 sm:pb-12" aria-label="Trusted product facts">
          <Container>
            <div className="glass-panel rounded-[1.9rem] px-4 py-5 sm:px-6 sm:py-6">
              <div className="flex flex-wrap gap-2">
                {trustPillars.map((pillar) => (
                  <span key={pillar} className="stat-pill rounded-full px-4 py-2 text-sm leading-6">
                    {pillar}
                  </span>
                ))}
              </div>
            </div>
          </Container>
        </section>

        <section id="features" className="section-anchor py-16 sm:py-20 lg:py-24">
          <Container>
            <SectionIntro
              eyebrow="Detection families"
              title="Signals grouped the way reviewers actually reason about risk"
              description="The current rule set covers naming, typing, error handling, security, concurrency, performance, data access, test quality, and local hallucination checks. The site keeps the view high-level, but the source guides define the detailed rule IDs and evidence model."
            />
            <FeatureGrid />
          </Container>
        </section>

        <section id="pipeline" className="section-anchor py-16 sm:py-20 lg:py-24">
          <Container>
            <SectionIntro
              eyebrow="Pipeline"
              title="A fast repository pass with a deliberately honest scope"
              description="deslop is structured as a staged Rust pipeline: repository discovery, syntax-tolerant parse, lightweight local indexing, and explainable heuristics. That split keeps the current tool fast while leaving room for deeper phases later."
            />
            <PipelineTabs />
          </Container>
        </section>

        <section id="use-cases" className="section-anchor py-16 sm:py-20 lg:py-24">
          <Container>
            <SectionIntro
              eyebrow="Use cases"
              title="Built for teams that review fast-moving Go code without pretending every signal is a verdict"
              description="The product story here follows the repo docs: deslop is best used as an early warning layer for code review, local audits, and automation, especially when AI-assisted output is part of the development loop."
            />

            <div className="mt-10 grid gap-4 lg:grid-cols-2">
              {useCases.map((useCase) => (
                <article key={useCase.title} className="glass-panel rounded-[1.75rem] p-6 sm:p-7">
                  <h3 className="text-2xl font-bold text-white">{useCase.title}</h3>
                  <p className="mt-3 text-base leading-7 text-[var(--muted)]">{useCase.description}</p>
                  <div className="mt-6 rounded-2xl border border-[var(--border-strong)] bg-[var(--accent-soft)] px-4 py-4 text-sm leading-6 text-[#dff6e4]">
                    {useCase.outcome}
                  </div>
                </article>
              ))}
            </div>

            <div className="mt-4 grid gap-4 lg:grid-cols-3">
              {noteCards.map((card) => {
                const Icon = card.icon

                return (
                  <article key={card.title} className="grid-panel rounded-[1.5rem] p-5">
                    <span className="flex h-11 w-11 items-center justify-center rounded-2xl border border-white/8 bg-white/5 text-[var(--accent)]">
                      <Icon className="h-5 w-5" aria-hidden="true" />
                    </span>
                    <h3 className="mt-4 text-xl font-bold text-white">{card.title}</h3>
                    <p className="mt-2 text-sm leading-6 text-[var(--muted)]">{card.description}</p>
                  </article>
                )
              })}
            </div>
          </Container>
        </section>

        <section id="quickstart" className="section-anchor py-16 sm:py-20 lg:py-24">
          <Container>
            <SectionIntro
              eyebrow="Quick start"
              title="The site is marketing-led, but the commands stay exact"
              description="Every command surfaced here comes straight from the README or implementation guide. The point is to move from first impression to first scan without inventing a platform the repository does not ship."
            />
            <QuickStart />
          </Container>
        </section>

        <section id="principles" className="section-anchor py-16 sm:py-20 lg:py-24">
          <Container className="grid gap-6 lg:grid-cols-[0.96fr_1.04fr] lg:items-start">
            <div>
              <SectionIntro
                eyebrow="Principles"
                title="Credibility matters more than sounding comprehensive"
                description="The guides are careful about limitations, so the website should be too. deslop is not presented as authoritative type checking or leak-proof concurrency analysis. It is a fast static signal layer meant to sharpen engineering attention."
              />

              <div className="mt-8 space-y-4">
                {principles.map((principle) => (
                  <article key={principle.title} className="glass-panel rounded-[1.5rem] p-6">
                    <h3 className="text-2xl font-bold text-white">{principle.title}</h3>
                    <p className="mt-3 text-sm leading-6 text-[var(--muted)] sm:text-base">{principle.description}</p>
                  </article>
                ))}
              </div>
            </div>

            <div className="glass-panel rounded-[2rem] p-6 sm:p-8">
              <p className="eyebrow">Benchmark reference</p>
              <h3 className="mt-5 text-3xl font-bold text-white sm:text-[2.4rem]">Fast enough for local loops, explicit enough for review.</h3>
              <p className="mt-4 max-w-2xl text-base text-[var(--muted)] sm:text-lg">
                The implementation guide documents a preferred baseline against a realistic local Go repository. The numbers belong here as evidence,
                not as a universal promise for every codebase shape.
              </p>

              <div className="mt-8 grid gap-4 sm:grid-cols-3">
                {metrics.map((metric) => (
                  <article key={metric.label} className="grid-panel rounded-[1.5rem] p-5">
                    <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--muted)]">{metric.label}</p>
                    <p className="mt-4 text-2xl font-bold text-white">{metric.value}</p>
                    <p className="mt-2 text-sm leading-6 text-[var(--muted)]">{metric.note}</p>
                  </article>
                ))}
              </div>

              <div className="mt-8 rounded-[1.7rem] border border-[var(--border-strong)] bg-[var(--accent-soft)] p-6">
                <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--accent)]">What this page will not claim</p>
                <p className="mt-3 max-w-2xl text-sm leading-7 text-[#def7e3] sm:text-base">
                  No authoritative Go type checking. No interprocedural proof. No guarantee that every flagged issue is wrong. The value is in the speed,
                  coverage, and clarity of the evidence you get back.
                </p>
              </div>
            </div>
          </Container>
        </section>

        <section className="pb-20 pt-6 sm:pb-24 lg:pb-32">
          <Container>
            <div className="glass-panel rounded-[2.2rem] p-8 sm:p-10 lg:p-12">
              <div className="grid gap-8 lg:grid-cols-[1fr_auto] lg:items-end">
                <div>
                  <span className="eyebrow">Open source from day one</span>
                  <h2 className="mt-5 max-w-3xl text-3xl font-bold text-white sm:text-5xl">
                    Keep the workflow local, keep the findings readable, keep the review bar higher than “it compiled.”
                  </h2>
                  <p className="mt-4 max-w-3xl text-base text-[var(--muted)] sm:text-lg">
                    deslop is already structured for extension, but the current landing page stays faithful to what the repo actually ships today:
                    a Rust CLI for Go repositories with explainable static signals and repeatable benchmarks.
                  </p>
                </div>

                <div className="flex flex-col gap-3 sm:flex-row lg:flex-col">
                  <a href="#quickstart" className="button-primary">
                    Run the commands
                    <ArrowRightIcon className="h-4 w-4" aria-hidden="true" />
                  </a>
                  <a href="#features" className="button-secondary">
                    Browse the signals
                  </a>
                </div>
              </div>
            </div>
          </Container>
        </section>
      </main>

      <footer className="border-t border-white/8 pb-10 pt-8 sm:pb-12">
        <Container className="grid gap-8 lg:grid-cols-[1.1fr_0.9fr]">
          <div>
            <p className="font-['Space_Grotesk'] text-2xl font-bold tracking-[-0.05em] text-white">deslop</p>
            <p className="mt-3 max-w-2xl text-sm leading-7 text-[var(--muted)] sm:text-base">
              A static analyzer for Go repositories that surfaces suspicious low-context or AI-assisted code patterns quickly, then hands the judgment back to the engineer.
            </p>

            <div className="mt-6 flex flex-wrap gap-2">
              {footerLinks.map((link) => (
                <a key={link.href} href={link.href} className="stat-pill rounded-full px-4 py-2 text-sm hover:text-white">
                  {link.label}
                </a>
              ))}
            </div>
          </div>

          <div className="grid gap-4 sm:grid-cols-3">
            {footerSources.map((source) => (
              <div key={source} className="grid-panel rounded-[1.4rem] p-4">
                <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.18em] text-[var(--accent)]">Source</p>
                <p className="mt-3 text-sm leading-6 text-[#d9e7dc]">{source}</p>
              </div>
            ))}
          </div>
        </Container>
      </footer>
    </div>
  )
}