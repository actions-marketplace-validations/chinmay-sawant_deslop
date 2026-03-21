import { Tab, TabGroup, TabList, TabPanel, TabPanels } from '@headlessui/react'

import { quickStartItems, siteMetadata } from '../../../content/site-content'

// Display order: GitHub Actions → crates.io → Binary → Scan
const TAB_ORDER = [2, 0, 1, 3]

export function QuickStart() {
  const orderedItems = TAB_ORDER.map((i) => quickStartItems[i])

  return (
    <TabGroup defaultIndex={0} className="mt-14">
      <TabList className="flex flex-wrap gap-3">
        {orderedItems.map((item, index) => (
          <Tab
            key={item.label}
            className="cursor-pointer rounded-full border border-[var(--border)] bg-[var(--panel-muted)] px-5 py-3 text-sm font-medium text-[var(--muted)] transition data-[hover]:border-[var(--border-strong)] data-[hover]:text-[var(--text)] data-[selected]:border-[var(--border-strong)] data-[selected]:bg-[var(--text)] data-[selected]:text-[var(--bg)]"
          >
            <span className="font-['IBM_Plex_Mono'] text-[0.68rem] uppercase tracking-[0.16em]">
              0{index + 1}
            </span>
            <span className="ml-2">{item.label}</span>
          </Tab>
        ))}
      </TabList>
      <TabPanels className="mt-8">
        {/* Tabs 0–2: standard layout */}
        {orderedItems.slice(0, 3).map((item) => (
          <TabPanel key={item.label} className="glass-panel rounded-[2.25rem] p-8 sm:p-10 lg:p-12">
            <div className="grid gap-10 lg:grid-cols-[1.15fr_0.85fr] lg:items-start">
              {/* Left: description */}
              <div>
                <span className="eyebrow">{item.channel}</span>
                <h3 className="mt-6 text-4xl leading-tight font-bold sm:text-[3rem]">{item.label}</h3>
                <p className="mt-5 max-w-2xl text-base leading-8 text-[var(--muted)] sm:text-lg">
                  {item.description}
                </p>
                {item.linkHref && (
                  <a
                    href={item.linkHref}
                    target="_blank"
                    rel="noreferrer"
                    className="button-secondary mt-8 inline-flex"
                  >
                    {item.linkLabel}
                  </a>
                )}
              </div>

              {/* Right: snippet */}
              <div className="grid-panel rounded-[1.8rem] p-6 sm:p-7">
                <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--muted)]">
                  {item.channel} setup
                </p>
                <div className="mt-6">
                  {item.showPrompt && item.snippet.length === 1 ? (
                    <div className="terminal-line font-['IBM_Plex_Mono'] text-[0.82rem] leading-7">
                      <span className="terminal-prompt">$</span>
                      <span className="terminal-copy break-all">{item.snippet[0]}</span>
                    </div>
                  ) : (
                    <pre className="overflow-x-auto whitespace-pre-wrap break-words font-['IBM_Plex_Mono'] text-[0.78rem] leading-7 text-[var(--text)]">
                      {item.snippet.join('\n')}
                    </pre>
                  )}
                </div>
              </div>
            </div>
          </TabPanel>
        ))}

        {/* Tab 3: Scan — shows scan command + install quick-refs */}
        {(() => {
          const scanItem = orderedItems[3]
          const cratesItem = quickStartItems[0]
          const binaryItem = quickStartItems[1]
          return (
            <TabPanel key={scanItem.label} className="glass-panel rounded-[2.25rem] p-8 sm:p-10 lg:p-12">
              <div className="grid gap-10 lg:grid-cols-[1.15fr_0.85fr] lg:items-start">
                {/* Left: description */}
                <div>
                  <span className="eyebrow">{scanItem.channel}</span>
                  <h3 className="mt-6 text-4xl leading-tight font-bold sm:text-[3rem]">{scanItem.label}</h3>
                  <p className="mt-5 max-w-2xl text-base leading-8 text-[var(--muted)] sm:text-lg">
                    {scanItem.description}
                  </p>
                  <p className="mt-4 text-sm leading-7 text-[var(--muted)]">
                    Make sure deslop is installed first — install via Cargo or grab a prebuilt binary.
                  </p>
                </div>

                {/* Right: scan command + install quick-refs */}
                <div className="flex flex-col gap-4">
                  {/* Scan command */}
                  <div className="grid-panel rounded-[1.8rem] p-6">
                    <p className="font-['IBM_Plex_Mono'] text-xs uppercase tracking-[0.2em] text-[var(--muted)]">
                      Scan command
                    </p>
                    <div className="terminal-line mt-5 font-['IBM_Plex_Mono'] text-[0.82rem] leading-7">
                      <span className="terminal-prompt">$</span>
                      <span className="terminal-copy">{scanItem.snippet[0]}</span>
                    </div>
                  </div>

                  {/* Install quick-refs */}
                  <div className="grid grid-cols-2 gap-3">
                    <div className="grid-panel rounded-[1.4rem] p-4">
                      <p className="font-['IBM_Plex_Mono'] text-[0.66rem] uppercase tracking-[0.16em] text-[var(--muted)]">
                        Via Cargo
                      </p>
                      <div className="terminal-line mt-3 font-['IBM_Plex_Mono'] text-[0.72rem] leading-6">
                        <span className="terminal-prompt">$</span>
                        <span className="terminal-copy break-all">{cratesItem.snippet[0]}</span>
                      </div>
                    </div>
                    <div className="grid-panel rounded-[1.4rem] p-4">
                      <p className="font-['IBM_Plex_Mono'] text-[0.66rem] uppercase tracking-[0.16em] text-[var(--muted)]">
                        Via Binary
                      </p>
                      <a
                        href={siteMetadata.github.releaseUrl}
                        target="_blank"
                        rel="noreferrer"
                        className="mt-3 block font-['IBM_Plex_Mono'] text-[0.72rem] leading-6 text-[var(--text)] underline underline-offset-2 transition hover:text-[var(--accent-strong)]"
                      >
                        v0.1.0 release assets →
                      </a>
                      <p className="mt-1 text-[0.68rem] leading-5 text-[var(--muted)]">
                        {binaryItem.snippet.slice(1, 3).join(', ')}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </TabPanel>
          )
        })()}
      </TabPanels>
    </TabGroup>
  )
}