# Tray — design sketch (pre-implementation)

The current `tray.rs` is a shape-only stub (`Tray`/`TrayBuilder` structs with
no-ops) so the public API is visible today. This document describes the real
implementation. Status and task breakdown live in `TODOS.md` Phase 4.

## Goals

1. **No GUI framework.** Backends: `ksni` (pure-Rust DBus SNI client) on
   Linux; `tray-icon` (native Win32) on Windows; macOS deferred in v0.1
   (needs winit/tao event-loop integration — see below).
2. **The host application owns the loop.** The tray must never start its own
   UI thread or require an event-loop harness at call sites. Everything is
   **poll-based**: menu activations are queued and the host pops them.
3. **Honest errors.** No menu host (e.g. GNOME without the AppIndicator
   extension) means `GuiUtilsError::Unsupported` with an actionable message,
   not a tray icon that silently never appears.

## Proposed API

```rust
let tray = TrayBuilder::new()
    .title("My App")
    .tooltip("Click the menu")
    .icon_from_file("/usr/share/icons/app.png")
    // Flat menu, ids the host understands:
    .menu_item("open", "Open window")
    .menu_item("settings", "Settings")
    .separator()
    .menu_item("quit", "Quit")
    .build()?;                      // GuiUtilsError; may be Unsupported

// In the host's own loop / polling thread:
while let Some(event) = tray.pop_event() {
    match event {
        TrayEvent::MenuItem { id, .. } if id == "quit" => break,
        TrayEvent::Activate => show_window(),
        _ => {}
    }
}

tray.hide();
```

Additions over the current stub: `build()` returns `Result` (backend/host
detection may fail), menu items with stable ids, `TrayEvent`, and a
`pop_event()` side channel. `set_tooltip` / `set_icon` keep their builder
shapes; a `set_menu` mutator supports re-rendering.

## Event plumbing (Linux, ksni)

```
host thread                    ksni service thread
-----------                    --------------------
TrayBuilder::build()
   |--- Arc<Mutex<TrayState>>---+
                              ksni::TrayMethods::spawn(&state)
tray.pop_event()                 |--- reads menu model from TrayState
   |<-- mpsc receiver ---+       |--- MenuItem activation callbacks:
                          +------|     state.lock().events.push(id)
```

- `TrayState` holds the menu model, static strings for ksni, and
  `SyncSender<TrayEvent>`.
- The `ksni::Tray` impl (`menu(&self) -> Vec<MenuItem<Self>>`,
  `activate(...)`, `icon_pixmap(...)`) reads from the shared state; item
  activation pushes the item's id into the queue.
- `pop_event()` does a non-blocking `try_recv()`; a blocking
  `wait_event(timeout)` variant may be added later.

## Event plumbing (Windows, tray-icon)

`tray-icon` exposes a global event receiver (`TrayIconEvent::receiver()`
returning a channel). `pop_event()` forwards `try_recv()` results from it.
Icon loading uses native image handles; menu events arrive with the item id
set by the builder, so the shared id scheme is preserved.

## macOS (deferred)

`tray-icon`'s macOS backend requires an existing event loop (winit/tao) on
the main thread. v0.1 ships a documented stub: `TrayBuilder::build()` maps
the capability gap to `GuiUtilsError::Unsupported` with a message naming the
limitation. Options tracked in TODOS: (a) integrate behind an explicit
feature flag that requires a compatible event loop, (b) revisit tray-icon
releases that lift the requirement.

## Runtime host detection (Linux check sketch)

```
DBus ping "org.kde.StatusNotifierWatcher"
   ├─ present         -> build normally
   ├─ absent          -> Unsupported("no StatusNotifier host; on GNOME install
                          the AppIndicator extension, or use KDE/Xfce/...")
   └─ error other..   -> Backend(cause)
```

## Testing plan

- Unit: menu model construction (ids, separators, order), event queue
  push/pop semantics, error mapping — fully headless.
- Interactive (`#[ignore]`): spawn a real icon on the current session, pop
  events, print for manual verification.
- CI: Linux unit tests; Windows/macOS `cargo check` so the cfg-split code
  always compiles (the invalid-`tray.rs` bug class this crate shipped with is
  exactly what per-target compilation catches).
