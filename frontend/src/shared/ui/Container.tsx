import type { ComponentPropsWithoutRef } from 'react'

import { cn } from '../lib/cn'

export function Container({ className, ...props }: ComponentPropsWithoutRef<'div'>) {
  return <div className={cn('mx-auto w-full max-w-[96rem] px-6 sm:px-10 lg:px-16', className)} {...props} />
}