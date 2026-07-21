# Weather Index
*Unified platform for Risk of Rain 2 runs*

# Features
- Rust backend
  - [axum](https://github.com/tokio-rs/axum) routing
  - [better-auth-rs](https://github.com/better-auth-rs/better-auth-rs)[^1] for auth
  - [sea-orm](https://github.com/SeaQL/sea-orm) for database handling
- SvelteKit frontend
  - [Geist](https://vercel.com/geist/introduction) design systems
  - [TailwindCSS](https://tailwindcss.com/) for styling
  - [Lucide](https://lucide.dev/) icons for non-brand icons
  - [Simple Icons](https://simpleicons.org/) for brand-icons
- Risk of Rain 2 mod
  - Tracks all sorts of data, from kills, to distance traveled, to stage order, to which items where picked up when
    - Full table can be found at [backend/src/entity/run_report.rs]
  - Data dumper to extract item data, enemy/survivor data, and more
- CORS support

# Installation
TODO: ACTUALLY PUBLISH THE FUCKING MOD
TODO: PUBLISH THE SITE TOO
1. Sign in on the [website](https://weather-index-ror2.vercel.app)
2. Ensure that [r2modman](https://old.thunderstore.io/c/riskofrain2/p/ebkr/r2modman/) is installed.
3. Make a new profile (or use an existing profile)
4. Install "Weather Index" from the `Online` tab
5. `Start Modded`
6. In the game's `Mod Settings`, find `WeatherIndex` and click `Link Account`. This will open your web browser for authentication.

# Project Structure
```bash
Weather-Index/
├── backend # This is where the BACKEND goes
├── data # This is where static DATA goes
├── frontend # This is where the FRONTEND goes
└── mod # This is where the MOD goes
```

# Development Setup
1. Clone from `ssh://git@codeberg.org/Shuflduf/Weather-Index.git`

[^1]: The official library is so buggy, this project instead uses [a fork](https://github.com/Shuflduf/better-auth-rs)
