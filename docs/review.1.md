# Source Code Review: webapp-akiapp

## 1. Project Overview
The project is a **Dioxus Fullstack** application serving as an "App List" portal for various platform-specific applications (Web, Android APK, and Linux AppImage). The architecture cleanly separates backends, reusable components, and view-level pages.

---

## 2. Architecture & Design
- **Fullstack Integration**: Effectively uses `#[cfg(feature = "server")]` to isolate server-side I/O (file system access, TOML parsing) from the client-side UI.
- **Componentization**: Components like `List`, `AppListRowCm`, and `Version` are well-isolated and reusable across different views (`Home` and `Devel`).
- **State Management**: Uses `Store` for the `AppDialog`, which is appropriate for managing a complex UI state like a modal that needs to be accessible from multiple list rows.

---

## 3. Code Quality & Idioms
- **Macro Usage**: The macros in `src/backends/conf.rs` (`fn_is_xxx_yyy!` and `fn_is_xxx!`) are a clever way to handle repetitive boolean checks for optional configuration fields, significantly reducing boilerplate.
- **Testing**: Excellent coverage of configuration serialization/deserialization in `conf.rs`, ensuring that the core data structure matches the expected TOML format.
- **Error Handling**: Generally good use of `anyhow` for context-rich error reporting on the server side.

---

## 4. Issues & Potential Improvements

### Hardcoded Configuration & Paths
- **Issue**: Absolute paths like `/opt/webapp-akiapp/web/config.toml` and `/opt/webapp-{name}/android/` are hardcoded in `src/backends/mod.rs`.
- **Impact**: This limits portability and makes the application difficult to run in a containerized or local development environment without matching the exact `/opt` structure.
- **Recommendation**: Use environment variables or a runtime configuration to define base directories.

### Hardcoded URLs
- **Issue**: `BASE_URL` in `src/components/list.rs` and `backend_url` in `src/main.rs` are hardcoded.
- **Impact**: Makes testing and multi-environment deployment (staging vs. production) difficult.
- **Recommendation**: Externalize these URLs using environment variables or a configuration service.

### Server-Side Efficiency
- **Issue**: `list_apps` reads and parses `config.toml` from the disk on **every** request.
- **Impact**: Increased disk I/O and CPU usage under high traffic.
- **Recommendation**: Cache the parsed configuration in memory (e.g., using `once_cell` or a global state managed by the server framework) and reload it only when necessary.

### Robustness in `find_fnm_appimage`
- **Issue**: `Version::parse(version_s.as_str())?` is used within a loop.
- **Impact**: If a single file in the directory has a name that doesn't follow the semver format perfectly, the entire `list_apps` API call will fail with an error.
- **Recommendation**: Use `filter_map` or `if let Ok(...)` to ignore files with invalid versions instead of failing the entire request.

### Dioxus Patterns
- **Issue**: `AppListRowCm` uses `use_store` to clone `app_info`, but `app_info` is already provided as a prop.
- **Recommendation**: Since the component doesn't appear to modify the `app_info` locally, using it directly from props or a simple `use_signal` would be more idiomatic and slightly more performant.
- **Issue**: `document::eval` is used for closing/opening the modal.
- **Recommendation**: While acceptable for native `<dialog>` elements, consider using Dioxus signals to control the `open` attribute of the dialog for a more declarative approach.

---

## 5. Security
- **Path Sanitization**: The backend constructs paths based on `conf_app.name()`. While this name currently comes from a trusted configuration file, if this were ever to be user-influenced, it could lead to directory traversal. Ensure the names are strictly validated before being used in `format!("/opt/webapp-{name}/...")`.

---
*Review Date: 2026-04-15*
