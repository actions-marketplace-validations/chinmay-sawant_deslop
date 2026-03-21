import { detectionFamilies } from '../../../content/site-content'

export function FeatureGrid() {
  return (
    <div className="mt-14 grid gap-5 md:grid-cols-2 xl:grid-cols-3">
      {detectionFamilies.map((family) => {
        const Icon = family.icon

        return (
          <article key={family.title} className="glass-panel rounded-[2rem] p-7 sm:p-8">
            <div className="flex items-center gap-4">
              <span className="flex h-12 w-12 items-center justify-center rounded-2xl border border-white/10 bg-white/3 text-white">
                <Icon className="h-6 w-6" aria-hidden="true" />
              </span>
            </div>

            <h3 className="mt-8 text-[1.95rem] leading-tight font-bold text-white">{family.title}</h3>
            <p className="mt-4 text-base leading-8 text-[var(--muted)]">{family.description}</p>

            <ul className="mt-8 flex flex-wrap gap-2.5 border-t border-white/8 pt-6">
              {family.rules.map((rule) => (
                <li
                  key={rule}
                  className="rounded-full border border-white/8 bg-white/3 px-3.5 py-1.5 text-[0.78rem] text-[#e9e9e9]"
                >
                  {rule}
                </li>
              ))}
            </ul>
          </article>
        )
      })}
    </div>
  )
}