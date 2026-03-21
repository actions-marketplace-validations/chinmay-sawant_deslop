import { Tab, TabGroup, TabList, TabPanel, TabPanels } from '@headlessui/react'

import { pipelineStages } from '../../../content/site-content'

export function PipelineTabs() {
  return (
    <TabGroup className="mt-10">
      <TabList className="flex flex-wrap gap-2">
        {pipelineStages.map((stage, index) => (
          <Tab
            key={stage.name}
            className="rounded-full border border-white/10 bg-white/4 px-4 py-2.5 text-sm font-medium text-[var(--muted)] transition data-[hover]:border-white/20 data-[hover]:text-white data-[selected]:border-[var(--border-strong)] data-[selected]:bg-[var(--accent-soft)] data-[selected]:text-white"
          >
            <span className="font-['IBM_Plex_Mono'] text-[0.68rem] uppercase tracking-[0.16em] text-[var(--accent)]">0{index + 1}</span>
            <span className="ml-2">{stage.name}</span>
          </Tab>
        ))}
      </TabList>

      <TabPanels className="mt-6">
        {pipelineStages.map((stage) => (
          <TabPanel key={stage.name} className="glass-panel rounded-[2rem] p-6 sm:p-8">
            <div className="grid gap-8 lg:grid-cols-[1.05fr_0.95fr]">
              <div>
                <span className="eyebrow">{stage.name}</span>
                <h3 className="mt-5 text-3xl font-bold text-white sm:text-[2.4rem]">{stage.summary}</h3>
                <p className="mt-4 max-w-2xl text-base text-[var(--muted)] sm:text-lg">{stage.detail}</p>
              </div>

              <div className="grid-panel rounded-[1.6rem] p-5 sm:p-6">
                <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--muted)]">Stage details</p>
                <ul className="mt-4 space-y-4">
                  {stage.bullets.map((bullet) => (
                    <li key={bullet} className="flex items-start gap-3 text-sm leading-6 text-[#d9e7dc] sm:text-base">
                      <span className="mt-2 h-2.5 w-2.5 rounded-full bg-[var(--accent)]" aria-hidden="true" />
                      <span>{bullet}</span>
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          </TabPanel>
        ))}
      </TabPanels>
    </TabGroup>
  )
}