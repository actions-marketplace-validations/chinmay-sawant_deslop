import { detectionFamilies } from '../../../content/site-content'

export function FeatureGrid() {
  return (
    <div className="mt-10 grid gap-4 md:grid-cols-2 xl:grid-cols-4">
      {detectionFamilies.map((family) => {
        const Icon = family.icon

        return (
          <article key={family.title} className="glass-panel rounded-[1.75rem] p-6">
            <div className="flex items-center justify-between gap-4">
              <span className="flex h-12 w-12 items-center justify-center rounded-2xl border border-[var(--border-strong)] bg-[var(--accent-soft)] text-[var(--accent)]">
                <Icon className="h-6 w-6" aria-hidden="true" />
              </span>
              <span className="font-['IBM_Plex_Mono'] text-[0.72rem] uppercase tracking-[0.18em] text-[var(--muted)]">Signal family</span>
            </div>

            <h3 className="mt-5 text-2xl font-bold text-white">{family.title}</h3>
            <p className="mt-3 text-sm leading-6 text-[var(--muted)]">{family.description}</p>

            <ul className="mt-5 flex flex-wrap gap-2">
              {family.rules.map((rule) => (
                <li
                  key={rule}
                  className="rounded-full border border-white/8 bg-white/4 px-3 py-1.5 font-['IBM_Plex_Mono'] text-[0.72rem] text-[#d8e5db]"
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