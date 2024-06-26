# Tauri IPC
> MoonZoon example

---

### Start:

1. `cargo install tauri-cli@=2.0.0-beta.12`
2. `cargo tauri dev`

Troubleshooting:
- In case of Tauri compilation errors, install system dependencies: https://beta.tauri.app/guides/prerequisites/

- Possible Tauri runtime errors in terminal of VSCode installed from Linux Snap package manager:
    ```
    Failed to load module "colorreload-gtk-module"

    /usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/WebKitNetworkProcess: symbol lookup error: /snap/core20/current/lib/x86_64-linux-gnu/libpthread.so.0: undefined symbol: __libc_pthread_init, version GLIBC_PRIVATE
    ```
    Fix it by installing VSCode directly from official `.deb` bundle or try to unset multiple env variables - more info in https://stackoverflow.com/questions/75921414/java-symbol-lookup-error-snap-core20-current-lib-x86-64-linux-gnu-libpthread

- There is a noticeable lag when clicking global menu items on macOS.
  - It seems to be a macOS feature, not a bug. The same lag happens even when e.g. closing a native macOS System Settings though its menu (File -> Close).
  - https://github.com/electron/electron/issues/6498
  - https://apple.stackexchange.com/questions/199890/why-do-menu-items-blink-twice-when-selected
  - https://apple.stackexchange.com/questions/293167/how-can-i-disable-menu-blinking-after-selecting-a-menu-item-in-macos

---

### Production build:

1. `cargo tauri build`
2. Runnable executable is in `target/release`
3. Installable bundles specific for the platform are in `target/release/bundle`

---

### Integration steps for a standard Tauri IPC example to make this example:

1. Install Tauri CLI: `cargo install tauri-cli@=2.0.0-beta.12`
2. `cargo tauri init`
3. App name: `Tauri IPC`
4. Window title: `Tauri IPC`
5. Web assets relative path: `../frontend_dist`
6. Dev server url: `http://localhost:8080`
7. Frontend dev command: `makers mzoon start`
8. Frontend build command: `makers mzoon build -r -f`
9. Add `"src-tauri"` to `Cargo.toml` workspace members.
10. Change `identifier` in `src-tauri/tauri.conf.json` to `"com.example.moonzoon.tauri-ipc"`
11. Set env var `WEBKIT_DISABLE_DMABUF_RENDERER=1` in `src-tauri/lib.rs` because WebKitGTK (2.42) is not compatible with NVIDIA drivers on Linux.
12. Enable `tauri` crate feature `linux-ipc-protocol` and `macos-private-api` in `src-tauri/Cargo.toml`.
13. Change `app.withGlobalTauri` in `src-tauri/tauri.conf.json` to `true`.
13. Add "macOSPrivateApi": true` in `app` in `src-tauri/tauri.conf.json`.
14. Add `"center": true` and `visible": false` to the first window config in `app.windows` in `src-tauri/tauri.conf.json`.
15. Add `"titleBarStyle": "Transparent"` to the first window config in `app.windows` in `src-tauri/tauri.conf.json`.
16. Add `"transparent": true` to the first window config in `app.windows` in `src-tauri/tauri.conf.json`.

### Transparent window

- https://beta.tauri.app/references/v2/config/#transparent
- From the doc above: _"Note that on macOS this requires the macos-private-api feature flag, enabled under tauri &gt; macOSPrivateApi. WARNING: Using private APIs on macOS prevents your application from being accepted to the App Store."_

### Title bar styling and customizations

- https://www.youtube.com/watch?v=zsaWFf2LEv4
- https://beta.tauri.app/references/v2/config/#titlebarstyle
- https://docs.rs/tauri-utils/2.0.0-beta.11/tauri_utils/enum.TitleBarStyle.html
- _Note_: To make the title bar transparent, you have to set `"macOSPrivateApi": true` like in the case of transparent window described above.
