import { cn } from '../lib/cn'

type SectionIntroProps = {
  eyebrow: string
  title: string
  description: string
  className?: string
}

export function SectionIntro({ eyebrow, title, description, className }: SectionIntroProps) {
  return (
    <div className={cn('max-w-3xl', className)}>
      <span className="eyebrow">{eyebrow}</span>
      <h2 className="mt-5 text-3xl leading-tight font-bold sm:text-4xl">{title}</h2>
      <p className="mt-4 max-w-2xl text-base text-[var(--muted)] sm:text-lg">{description}</p>
    </div>
  )
}