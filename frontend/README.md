# xtrace frontend

React dashboard for the [xtrace](https://github.com/lipish/xtrace) observability service.

Built with Vite, React, TypeScript, Tailwind CSS, and shadcn/ui.

## Development

```bash
npm install
npm run dev
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VITE_XTRACE_BASE_URL` | `http://127.0.0.1:8742` | xtrace server URL |
| `VITE_XTRACE_API_TOKEN` | — | Bearer token (`API_BEARER_TOKEN`) |

Create a `.env.local` file in this directory:

```
VITE_XTRACE_BASE_URL=http://127.0.0.1:8742
VITE_XTRACE_API_TOKEN=your_token_here
```

## Build

```bash
npm run build   # outputs to dist/
npm run preview # preview the production build
```

## Pages

- **Dashboard** — overview stats and trace/cost trend charts
- **Traces** — paginated trace list with search, click to view detail
- **Trace Detail** — full trace view with observation tree (inputs, outputs, token usage, latency)
