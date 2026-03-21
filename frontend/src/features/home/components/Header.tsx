import { Disclosure, DisclosureButton, DisclosurePanel } from '@headlessui/react'
import { Bars3Icon, ArrowUpRightIcon, XMarkIcon } from '@heroicons/react/24/outline'

import { navigation } from '../../../content/site-content'
import { cn } from '../../../shared/lib/cn'
import { Container } from '../../../shared/ui/Container'

function Logo() {
  return (
    <a href="#top" className="flex items-center gap-3">
      <span className="flex h-11 w-11 items-center justify-center rounded-2xl border border-white/10 bg-white/5 font-['Space_Grotesk'] text-lg font-bold text-white shadow-[0_18px_40px_rgba(0,0,0,0.25)]">
        d/
      </span>
      <span className="flex flex-col leading-none">
        <span className="font-['Space_Grotesk'] text-lg font-bold tracking-[-0.05em] text-white">deslop</span>
        <span className="font-['IBM_Plex_Mono'] text-[0.67rem] uppercase tracking-[0.2em] text-[var(--muted)]">
          Go static analyzer
        </span>
      </span>
    </a>
  )
}

const navLinkClassName =
  'rounded-full px-4 py-2 text-sm font-medium text-[var(--muted)] transition hover:bg-white/5 hover:text-white focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]'

export function Header() {
  return (
    <Disclosure as="header" className="sticky top-0 z-50 border-b border-white/8 bg-[rgba(7,17,13,0.72)] backdrop-blur-xl">
      {({ open }) => (
        <>
          <Container className="flex items-center justify-between gap-4 py-4">
            <Logo />

            <nav className="hidden items-center gap-1 md:flex">
              {navigation.map((item) => (
                <a key={item.href} href={item.href} className={navLinkClassName}>
                  {item.label}
                </a>
              ))}
            </nav>

            <div className="hidden md:block">
              <a href="#quickstart" className="button-primary">
                Start with scan
                <ArrowUpRightIcon className="h-4 w-4" aria-hidden="true" />
              </a>
            </div>

            <DisclosureButton
              className="flex h-11 w-11 items-center justify-center rounded-full border border-white/10 bg-white/5 text-white transition hover:bg-white/10 md:hidden"
              aria-label={open ? 'Close navigation' : 'Open navigation'}
            >
              {open ? <XMarkIcon className="h-5 w-5" aria-hidden="true" /> : <Bars3Icon className="h-5 w-5" aria-hidden="true" />}
            </DisclosureButton>
          </Container>

          <DisclosurePanel className="border-t border-white/8 md:hidden">
            <Container className="pb-5">
              <div className="grid-panel rounded-3xl p-3">
                <div className="flex flex-col gap-1">
                  {navigation.map((item) => (
                    <a key={item.href} href={item.href} className={cn(navLinkClassName, 'px-4 py-3')}>
                      {item.label}
                    </a>
                  ))}
                </div>
                <a href="#quickstart" className="button-primary mt-3 w-full">
                  Start with scan
                </a>
              </div>
            </Container>
          </DisclosurePanel>
        </>
      )}
    </Disclosure>
  )
}