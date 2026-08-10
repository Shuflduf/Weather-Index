# Weather Index
*Unified platform for Risk of Rain 2 runs*

<p align="center">
  <a href="https://wi.shuflduf.xyz">
    <img width="128" height="128" alt="icon_full" src="https://github.com/user-attachments/assets/ab6a0cff-afbf-468f-a84a-ceda9f96a2fd" />
  </a>
</p>
<p align="center">https://wi.shuflduf.xyz/</p>

# Features
- Rust backend
  - [axum](https://github.com/tokio-rs/axum) routing
  - [better-auth-rs](https://github.com/better-auth-rs/better-auth-rs) [^1] for auth
    - OAuth providers: Discord, Google, GitHub, [Hack Club Auth](https://auth.hackclub.com/), and Slack (Hack Club workspace)
  - [sea-orm](https://github.com/SeaQL/sea-orm) for database handling
  - Full documented API routes, docs available on the frontend
- SvelteKit frontend
  - [Geist](https://vercel.com/geist/introduction) design systems
  - [TailwindCSS](https://tailwindcss.com/) for styling
  - [Lucide](https://lucide.dev/) icons for non-brand icons
  - [Simple Icons](https://simpleicons.org/) for brand-icons
  - [Scalar](https://scalar.com/) for API documentation
  - Interactive guide to get started with Risk of Rain 2 modding and Weather Index
- Risk of Rain 2 mod
  - Tracks all sorts of data, from kills, to distance traveled, to stage order, to which items where picked up and when
    - Full table can be found [here](backend/src/entity/run_report.rs)
  - Data dumper to extract item data, enemy/survivor data, and more
- CORS support

# Screenshots
<details>
<summary>Click to view screenshots</summary>
  
## Main run report listing
<img width="2256" height="1504" alt="image" src="https://github.com/user-attachments/assets/75ec7473-6254-4e31-a9ef-ca689573af46" />

## Properties to view, filter, and sort by
<img width="1524" height="508" alt="image" src="https://github.com/user-attachments/assets/8b08937e-1b08-4f78-a6c2-b7f52aae283a" />

## Stats on played survivors and difficulties
<img width="1304" height="1147" alt="image" src="https://github.com/user-attachments/assets/f0b67620-c6e2-47ed-bfc7-095beb2deb97" />

## Scalar API docs
<img width="2256" height="1504" alt="image" src="https://github.com/user-attachments/assets/be4472c3-7f7c-491f-85b0-62c70cad2034" />

## Introduction to modding guide
<img width="837" height="665" alt="image" src="https://github.com/user-attachments/assets/3f88e5b7-559b-4745-8760-964bf53b6eb5" />

## Settings page
<img width="1095" height="1403" alt="image" src="https://github.com/user-attachments/assets/51b1792a-2888-4f1b-b124-085bf7b4e333" />

## Profile page
<img width="1957" height="1133" alt="image" src="https://github.com/user-attachments/assets/abd98ee7-ccaf-4e6a-a8ce-b0e3d90d4c64" />
</details>


# Installation
*Full interactive guide is available on the website [here](https://wi.shuflduf.xyz/guide)*
1. Sign in on the [website](https://weather-index-ror2.vercel.app)
2. Ensure that [r2modman](https://old.thunderstore.io/c/riskofrain2/p/ebkr/r2modman/) is installed.
3. Make a new profile (or use an existing profile)
4. Install "WeatherIndex" from the `Online` tab
5. `Start Modded`
6. In the game's `Mod Settings`, find `WeatherIndex` and click `Link Account`. This will open your web browser for authentication.

# Project Structure [^2]
```bash
Weather-Index/
├── backend # This is where the BACKEND goes
├── data # This is where static DATA goes
├── frontend # This is where the FRONTEND goes
└── mod # This is where the MOD goes
```

# Development Setup
For all of the components, you will need a clone of `https://github.com/Shuflduf/Weather-Index.git`
## Backend
1. Make `backend/.env` from `backend/.env.example` and fill in all the fields
2. Run `cargo build` in `backend/` to install the dependencies
3. Run `cargo run` in `backend/` to start the backend on http://localhost:3000
## Frontend
1. Make `frontend/.env` from `frontend/.env.example` and fill in all the fields
2. Run `deno install` in `frontend/` to install the dependencies
3. Run `deno run dev` in `frontend/` to start the frontend on http://localhost:5173
## Mod
1. Run `dotnet build -c Release` in `mod/` to install the dependencies, build the mod, and package it.
2. In your r2modman profile, click `Settings` and click `Import Local Mod`
3. Select the generated zip under `mod/bin/Release/netstandard2.1/WeatherIndex_[VERSION].zip`
4. `Start Modded`
5. Change the `Debug > Backend URL` option to `http://localhost:3000` if interacting with the backend (which you probably are)

[^1]: The official library is so buggy, this project instead uses [a fork](https://github.com/Shuflduf/better-auth-rs)
[^2]: For legal reasons this is a joke
