# reverse-shell-generator-desktop

Desktop port of [0dayCTF/reverse-shell-generator](https://github.com/0dayCTF/reverse-shell-generator) (hosted at [revshells.com](https://revshells.com)).

Built with **[Tauri 2](https://tauri.app/)** so the UI runs in the **system WebView** (WebView2 / WKWebView / WebKitGTK) — low memory, no Chromium bundle.

| Platform | Web engine |
| --- | --- |
| Windows | WebView2 (Edge) |
| macOS | WKWebView |
| Linux | WebKitGTK |

## Features

- Same reverse / bind / MSFVenom / HoaxShell / assembled payload UI as the web app
- Offline-friendly download & raw payload (no Netlify functions required)
- LocalStorage for IP / port / theme persistence
- Default HTTP(S) proxy: `127.0.0.1:7897` (Clash / V2Ray mixed port)

## Prebuilt binaries

GitHub Actions builds **3 platforms × 2 architectures** on every push to `main` and on version tags:

| | x64 | arm64 |
| --- | --- | --- |
| **Windows** | ✅ | ✅ |
| **macOS** | ✅ | ✅ |
| **Linux** | ✅ | ✅ |

Artifacts are uploaded per-target as `reverse-shell-generator-<platform>-<arch>`.

### Automatic releases

| Trigger | Release |
| --- | --- |
| Push to `main` | Updates prerelease tag **`continuous`** with the latest installers |
| Tag `v*` (e.g. `v1.0.0`) | Publishes a normal GitHub Release for that version |

Installers: `.msi` / `.exe` (Windows), `.dmg` (macOS), `.AppImage` / `.deb` (Linux).

## Network proxy

On launch the app sets standard proxy environment variables to:

```text
http://127.0.0.1:7897
```

with `NO_PROXY=localhost,127.0.0.1,::1`.

Override at runtime:

```bash
# custom proxy
RSG_HTTP_PROXY=http://127.0.0.1:7890 ./reverse-shell-generator

# or use the usual vars
HTTP_PROXY=http://127.0.0.1:7890 HTTPS_PROXY=http://127.0.0.1:7890 ./reverse-shell-generator
```

> Note: the payload generator itself is offline. The proxy mainly affects any outbound requests made by the WebView (e.g. remote assets / future features). System WebView stacks may still use OS-level proxy settings depending on the platform.

## Develop

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) stable
- Platform WebView deps:
  - **macOS**: Xcode CLT
  - **Windows**: [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) + VS Build Tools
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`

### Run / build

```bash
# Desktop deps only (recommended; skips Parcel / native lmdb)
npm install --omit=optional

# Also install Parcel if you need the web/Netlify production bundle
# npm install

# dev (opens the desktop window)
npm run desktop:dev
# or
cargo tauri dev

# release build for the host machine
npm run desktop:build
# or
cargo tauri build
```

Release artifacts land under:

```text
src-tauri/target/release/bundle/
```

### Cross-target example (Apple Silicon → Intel macOS)

```bash
rustup target add x86_64-apple-darwin
cargo tauri build --target x86_64-apple-darwin
```

## Web / Docker (upstream)

The original static site + Netlify functions still work:

```bash
# static bundle (Parcel)
npm run build

# Docker (nginx)
docker build -t reverse_shell_generator .
docker run -d -p 80:80 reverse_shell_generator
```

For Netlify function / raw-link development:

```bash
npx netlify dev
```

## License

Same as upstream — see [LICENSE](./LICENSE).

## Upstream

Forked from https://github.com/0dayCTF/reverse-shell-generator

## Contributors ✨

Upstream project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. See the original repository for the full list.
