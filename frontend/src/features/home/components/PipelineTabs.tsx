import { Tab, TabGroup, TabList, TabPanel, TabPanels } from '@headlessui/react'

import { pipelineStages } from '../../../content/site-content'

export function PipelineTabs() {
  return (
    <TabGroup className="mt-14">
      <TabList className="flex flex-wrap gap-3">
        {pipelineStages.map((stage, index) => (
          <Tab
            key={stage.name}
            className="rounded-full border border-white/10 bg-white/2 px-5 py-3 text-sm font-medium text-[var(--muted)] transition data-[hover]:border-white/20 data-[hover]:text-white data-[selected]:border-white/20 data-[selected]:bg-white data-[selected]:text-black"
          >
            <span className="font-['IBM_Plex_Mono'] text-[0.68rem] uppercase tracking-[0.16em]">0{index + 1}</span>
            <span className="ml-2">{stage.name}</span>
          </Tab>
        ))}
      </TabList>

      <TabPanels className="mt-8">
        {pipelineStages.map((stage) => (
          <TabPanel key={stage.name} className="glass-panel rounded-[2.25rem] p-8 sm:p-10 lg:p-12">
            <div className="grid gap-10 lg:grid-cols-[1.1fr_0.9fr]">
              <div>
                <span className="eyebrow">{stage.name}</span>
                <h3 className="mt-6 max-w-3xl text-4xl leading-tight font-bold text-white sm:text-[3rem]">{stage.summary}</h3>
                <p className="mt-5 max-w-2xl text-base leading-8 text-[var(--muted)] sm:text-lg">{stage.detail}</p>
              </div>

              <div className="grid-panel rounded-[1.8rem] p-6 sm:p-7">
                <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--muted)]">Stage details</p>
                <ul className="mt-5 space-y-5">
                  {stage.bullets.map((bullet) => (
                    <li key={bullet} className="flex items-start gap-3 text-sm leading-7 text-[#ececec] sm:text-base">
                      <span className="mt-2 h-2.5 w-2.5 rounded-full bg-white" aria-hidden="true" />
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