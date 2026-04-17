<h4 align="right"><strong>English</strong> | <a href="./README_CN.md">简体中文</a></h4>

<p align="center">
    <img src="https://s2.loli.net/2023/10/23/MLfhA2owPCacmyU.png" width="138" alt="AChat logo"/>
</p>

<h1 align="center">AChat</h1>

<p align="center">
    <strong>A local-first, team-ready AI agent workspace.</strong>
</p>

<div align="center">
    <a href="https://twitter.com/AprilNEA" target="_blank">
        <img alt="twitter" src="https://img.shields.io/badge/follow-AprilNEA-green?style=flat-square&logo=Twitter"/>
    </a>
    <a href="https://github.com/AprilNEA/ChatGPT-Admin-Web/commits" target="_blank">
        <img alt="GitHub commit" src="https://img.shields.io/github/commit-activity/m/AprilNEA/ChatGPT-Admin-Web?style=flat-square"/>
    </a>
    <a href="./LICENSE.md" target="_blank">
        <img alt="license" src="https://img.shields.io/badge/license-MIT-blue?style=flat-square"/>
    </a>
    <img alt="status" src="https://img.shields.io/badge/v4-under%20construction-orange?style=flat-square"/>
</div>

<br/>

<p align="center">
    <img src="./docs/assets/coming-soon-banner.svg" alt="AChat v4 — Coming Soon" width="100%"/>
</p>

---

## About

**AChat v4** is a complete rewrite, repositioning the project as a **local-first, team-ready AI agent workspace**.

- **Local-first.** Your data lives on your machine in SQLite. Work offline, own your history.
- **Team-ready.** Optional sync to a self-hosted cloud server lets your team share conversations and collaborate.
- **Agents everywhere.** Run agents locally in the desktop app, in the browser against the cloud, or offload heavy jobs from desktop to cloud compute — all driven by the same Rust agent runtime.
- **Bring your own models.** First-class support for Ollama (local), OpenAI, Anthropic, and any OpenAI-compatible endpoint.

## Tech Stack

| Layer                | Tech                                         |
|----------------------|----------------------------------------------|
| Cloud server         | Rust · Axum · PostgreSQL · sqlx              |
| Desktop app          | Rust · Tauri 2 · SQLite (rusqlite, bundled)  |
| Agent runtime        | Shared Rust crate — runs in both desktop and server |
| Sync engine          | Hybrid Logical Clock (HLC) + last-write-wins |
| Web / Desktop UI     | React 19 · Vite 6 · TypeScript · Tailwind 4 · Radix UI |
| Routing              | react-router v7                              |
| State                | Zustand                                      |
| Monorepo             | Cargo workspace + pnpm + Turborepo           |

## Status

v4 is **under active construction**. The monorepo scaffold, crate boundaries, and platform bridge are in place; core features (agent runtime, providers, sync) are being implemented next.

Progress can be tracked in the [commit log](https://github.com/AprilNEA/ChatGPT-Admin-Web/commits/v4).

## Project History

AChat began as **ChatGPT Admin Web (CAW)** — a self-hosted ChatGPT front-end with user management, payments, and admin tooling. Over time it evolved through several major revisions before being reimagined as a local-first agent workspace.

| Version                                                       | Status              | Stack                      | Note                                    |
|---------------------------------------------------------------|---------------------|----------------------------|-----------------------------------------|
| [v4](https://github.com/AprilNEA/ChatGPT-Admin-Web/tree/v4)   | In development      | Rust · Tauri · Axum · React | Local-first agent workspace (current)   |
| [v3.2](https://github.com/AprilNEA/ChatGPT-Admin-Web/tree/v3.2) | Long-term support   | Next.js · NestJS · Prisma   | Final release of the admin-panel era    |
| [v3](https://github.com/AprilNEA/ChatGPT-Admin-Web/tree/v3)   | Superseded by v3.2  | Next.js · NestJS · Prisma   | Full rewrite with modern tech stack     |
| [v2](https://github.com/AprilNEA/ChatGPT-Admin-Web/tree/v2)   | Deprecated          | Next.js · PostgreSQL        | Flawed design                           |
| [v1](https://github.com/AprilNEA/ChatGPT-Admin-Web/tree/v1)   | No longer updated   | Next.js · Redis             | Initial Redis-backed prototype          |

Archived READMEs from earlier versions live under [`docs/history/`](./docs/history/).

## Contributors

<a href="https://github.com/AprilNEA/ChatGPT-Admin-Web/graphs/contributors">
    <img src="https://contrib.rocks/image?repo=AprilNEA/ChatGPT-Admin-Web" alt="Contributors"/>
</a>

## Donate

Thank you for the inspiration to keep this project going.

[GitHub Sponsor](https://github.com/sponsors/AprilNEA) · [Afdian](https://afdian.net/a/aprilnea)

## License

[MIT](./LICENSE.md) © AprilNEA
