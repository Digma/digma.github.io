# Gael.io

[![Deploy to GitHub Pages](https://github.com/Digma/digma.github.io/actions/workflows/deploy.yml/badge.svg)](https://github.com/Digma/digma.github.io/actions/workflows/deploy.yml)

## Setup

```sh
npm i
```

## Develop

Setup Rust and npm

Then

```sh
cargo install rsw
cargo install wasm-pack
npm run dev
```

## 🚀 Project Structure

You'll see the following folders and files:

```text
├── public/
├── src/
│   ├── components/
│   ├── content/
│   ├── images/
│   ├── layouts/
│   └── pages/
├── astro.config.mjs
├── README.md
├── package.json
└── tsconfig.json
```

## 🧞 Commands

All commands are run from the root of the project, from a terminal:

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `npm run astro -- --help` | Get help using the Astro CLI                     |

