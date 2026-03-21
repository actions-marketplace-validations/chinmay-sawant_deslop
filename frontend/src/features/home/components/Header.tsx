import { Disclosure, DisclosureButton, DisclosurePanel } from '@headlessui/react'
import { Bars3Icon, ArrowUpRightIcon, XMarkIcon } from '@heroicons/react/24/outline'

import { navigation } from '../../../content/site-content'
import { cn } from '../../../shared/lib/cn'
import { Container } from '../../../shared/ui/Container'

function Logo() {
  return (
    <a href="#top" className="flex items-center gap-3">
      <span className="flex h-11 w-11 items-center justify-center rounded-2xl border border-white/12 bg-white/3 font-['Space_Grotesk'] text-lg font-bold text-white">
        d/
      </span>
      <span className="flex flex-col leading-none">
        <span className="font-['Space_Grotesk'] text-lg font-bold tracking-[-0.05em] text-white">deslop</span>
        <span className="text-[0.76rem] tracking-[0.02em] text-[var(--muted)]">
          Static analysis platform
        </span>
      </span>
    </a>
  )
}

const navLinkClassName =
  'rounded-full px-4 py-2 text-sm font-medium text-[var(--muted)] transition hover:text-white focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]'

export function Header() {
  return (
    <Disclosure as="header" className="sticky top-0 z-50 border-b border-white/8 bg-[rgba(5,5,5,0.82)] backdrop-blur-xl">
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
                Quick start
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
              <div className="glass-panel rounded-3xl p-3">
                <div className="flex flex-col gap-1">
                  {navigation.map((item) => (
                    <a key={item.href} href={item.href} className={cn(navLinkClassName, 'px-4 py-3')}>
                      {item.label}
                    </a>
                  ))}
                </div>
                <a href="#quickstart" className="button-primary mt-3 w-full">
                  Quick start
                </a>
              </div>
            </Container>
          </DisclosurePanel>
        </>
      )}
    </Disclosure>
  )
}