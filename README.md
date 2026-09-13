# didrit-watcher

Autonomous background monitor written in Rust that tracks updates on mathematical course portals, performs differential analysis using Google Gemini, and delivers rich push notifications via ntfy.

## Features

- Non-intrusive scraping with automatic encoding detection (Windows-1252 / ISO-8859-1 to UTF-8).
- Content-addressable state tracking using SHA-256 digests.
- Zero unnecessary AI API calls when no content diff is detected.
- Semantic extraction of homework, evaluations, and attached files with Gemini Flash Lite.
- Rich ntfy notifications: direct click-through URLs, action buttons for attached PDFs, Markdown formatting, priority levels, and topic tags.
- Containerized multi-stage Docker build for minimal image footprint.

## Architecture

```
didrit-watcher/
├── src/
│   ├── config/     Configuration loader (.env)
│   ├── fetcher/    Resilient HTTP client
│   ├── parser/     HTML DOM extractor and URL resolver
│   ├── storage/    JSON state persistence engine
│   ├── diff/       Differential detector
│   ├── ai/         Gemini API client & prompt builder
│   ├── notifier/   ntfy client & rich payload builder
│   └── service/    Orchestration daemon
└── tests/          Unit and integration test suites
```

## Configuration

Copy `.env.example` to `.env` and fill in the required parameters:

```env
GEMINI_API_KEY=AIzaSy...
GEMINI_MODEL=gemini-2.5-flash-lite
NTFY_URL=https://ntfy.sh
NTFY_TOPIC=didrit
NTFY_TOKEN=
TARGET_URLS=https://example.com/course1.htm,https://example.com/course2.htm
CHECK_INTERVAL_SECONDS=3600
STATE_FILE_PATH=/app/data/state.json
RUN_ONCE=false
```

## Deployment

### Docker Compose

```bash
docker compose up -d --build
```

### Local Execution (Cargo)

```bash
cargo run --release
```

### Running Tests

```bash
cargo test
```
