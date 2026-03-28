## OxiCloud v0.5.3 — Security, Stability & Kubernetes Ready

A community-powered release with **42 commits** from **7 contributors**, touching **106 files** with nearly **2,000 lines of improvements**. This release focuses on **critical security hardening**, **memory & reliability optimizations**, **Kubernetes-native deployment via Helm**, and a wave of **UI/UX and WebDAV fixes** that dramatically improve the day-to-day experience.

---

### Highlights

- **Kubernetes Helm Chart** — First-class Kubernetes deployment with a full Helm chart, including optional WOPI integration
- **SQLx Migration System** — Replaced custom schema loader with `sqlx::migrate!()` for robust, versioned database migrations
- **Security Advisory Fix** — Patched RUSTSEC-2026-0037 (quinn-proto) to eliminate a known vulnerability
- **Folder Ownership Verification** — Files can no longer be moved to another user's folder, closing a critical access control gap
- **Drag & Drop into Breadcrumbs** — Drag files directly into breadcrumb folders for faster file organization
- **Download Progress for Large Files** — New progress bar for files >2GB with correct 64-bit math
- **Thumbnail Timeout Protection** — Large image processing now has configurable timeouts to prevent server hangs

---

### Features

- **SQL migration system using `sqlx::migrate!()`** — Automatic, versioned schema migrations on startup replace the previous manual schema loader (@jaredwolff — #191)
- **Helm chart for Kubernetes deployment** — Full Helm chart with configurable values, optional WOPI sidecar, and comprehensive documentation (@nk-designz — #198)
- **Drag & drop files into breadcrumb folders** — Move files by dragging them onto any breadcrumb folder in the navigation bar (@EdouardVanbelle — #238)
- **Download progress bar for files >2GB** — Inline viewer now shows real-time download progress with correct 64-bit float division, avoiding 32-bit overflow (@BillionClaw — #227)
- **Thumbnail generation timeout protection** — Configurable timeout (default 30s) prevents large image processing from hanging the server indefinitely (@DioCrafts — #242)
- **Add missing home icon** — Breadcrumb now displays the proper home icon for root folder navigation (@EdouardVanbelle — #235)

### Security & Access Control

- **Fix RUSTSEC-2026-0037** — Updated `quinn-proto` to 0.11.14 to patch a known security advisory (@jaredwolff — b6bcb7d)
- **Verify target folder ownership on file move** — Moving a file now validates that the caller owns the destination folder, preventing cross-user file injection (@BillionClaw — #224)
- **Enforce storage quota on WebDAV PUT uploads** — WebDAV uploads now check storage quota before persisting, returning 507 Insufficient Storage when exceeded — previously only REST and chunked uploads had this check (@BillionClaw — #220)
- **Cap admin initial quota to available disk space** — User creation no longer sets quotas exceeding actual available disk space (@BillionClaw — #226)
- **Resolve CSP blocking and session refresh loop** — Fixed Content Security Policy violations blocking inline styles and an infinite session refresh loop (@BillionClaw — #211)

### Performance & Memory

- **Drop encoded image data after decoding** — Explicitly frees the original encoded buffer after image decoding, reducing peak memory consumption during thumbnail generation by the original file size (@BillionClaw — #228)
- **Thumbnail generation timeout** — Wraps `spawn_blocking` in `tokio::time::timeout` so a single slow image can't block the thumbnail pipeline (@DioCrafts — #242)

### Bug Fixes

**Nextcloud Compatibility:**
- **Fix Nextcloud sync conflict** — Replaced static UUID-based ETags with content-hash ETags, resolving persistent sync conflicts in the Nextcloud desktop and mobile clients (@jaredwolff — #207)

**WebDAV:**
- **Preserve correct status codes for rename/move failures** — AlreadyExists→409, NotFound→404, AccessDenied→403 instead of blanket 500 errors (@BillionClaw — #222)
- **Enforce storage quota on PUT uploads** — Closes a gap where WebDAV could bypass quota checks (@BillionClaw — #220)

**Files & Storage:**
- **Batch folder deletion fails** — Added debug logging to diagnose and fix batch trash operation failures (@BillionClaw — #216)
- **Improve error messages for file/folder already exists** — More descriptive error messages when duplicate file/folder names are encountered (@BillionClaw — #225)
- **Correct shared link URL to include `/api` prefix** — Shared links previously generated 404 URLs missing the API path prefix (@BillionClaw — #223)

**Calendar & Contacts:**
- **Change calendar `owner_id` from String to Uuid** — Aligns calendar ownership with the native UUID type used everywhere else, fixing lookup failures (@BillionClaw — #208)
- **Allow RGBA colors in calendar events** — Calendar color validation now accepts RGBA format in addition to RGB (@JVMerkle — #202)

**Trash:**
- **Add missing display fields to `TrashedItemDto`** — Added `category`, `icon_class`, and `icon_special_class` fields so the trash view renders file type information correctly (@BillionClaw — #221)

**UI/UX:**
- **Resolve broken menu navigation** — Fixed menu items not responding to clicks (@BillionClaw — #212)
- **Resolve dark mode toggle and file search errors** — Dark mode toggle now derives state from localStorage; empty folder_id no longer causes search errors (@BillionClaw — #218)
- **Photos view bleeding into trash view** — Fixed CSS isolation issue where photos grid styles leaked into the trash panel (@jaredwolff — #196)
- **Align size values in table view** — File sizes now use `tabular-nums` for proper column alignment (@BillionClaw — #219)
- **WOPI public base URL for Docker** — Added `OXICLOUD_WOPI_PUBLIC_BASE_URL` env var support so WOPI document editing works behind reverse proxies in Docker (@BillionClaw — #234)

**Internationalization:**
- **Use translation keys for upload notification titles** — Replaced hardcoded English/Spanish strings with proper i18n lookup (@BillionClaw — #217)
- **Add missing `dialogs.share_folder` translation key** — Added to all 14 locale files, fixing share dialog failures for folders (@BillionClaw — #215)

**Build, CI & Deployment:**
- **Add PostgreSQL service to Docker publish workflow** — The Docker Hub release CI job was failing because it lacked the PostgreSQL service required by tests, causing missing container images for v0.5.2 (@BillionClaw — #214)
- **Remove `target-cpu=native` from Dockerfile** — Ensures Docker images are portable across different CPU architectures (@jaredwolff — a0ee538)
- **ARMv7 32-bit compilation overflow** — Fixed integer overflow on 32-bit ARM targets (@BillionClaw — #209)
- **Resolve clippy warnings and rustfmt issues for CI compliance** — Cleaned up all remaining linting issues (@zjean — #188)
- **Update CI references from `db/schema.sql` to sqlx migrations** — Aligned CI pipelines with the new migration system (@jaredwolff — f6e2b30)
- **Add pre-commit checks to CLAUDE.md** — Documented required `cargo fmt` + `cargo clippy` checks (@jaredwolff — #192)

**Migrations:**
- **Add ALTER TABLE fallback for `media_sort_date` column** — Handles pre-existing tables gracefully during migration (@jaredwolff — #195)

### Documentation

- **Add feature status table to README** — Clear overview of which features are stable, beta, or planned (@BillionClaw — #210)
- **Fix incorrect path in development guide** — Corrected branch path references in CONTRIBUTING.md (@BillionClaw — #213)
- **Helm chart documentation** — Comprehensive deployment guide for Kubernetes users (@nk-designz — #198)
- **Update README.md & example.env** — Improved documentation for remote access setup (@raenur — #197)

### Developer Experience

- **Dev-mode static assets without cache** — When `PROFILE=dev`, static assets are served directly from `/static` with no caching, enabling faster frontend iteration (@EdouardVanbelle — #236)
- **Remove duplicate breadcrumb home-folder code** — Refactored redundant logic in breadcrumb handling (@EdouardVanbelle — 276b9ff)
- **Apply rust format + fix clippy warning** — Code style cleanup (@EdouardVanbelle — #240)

---

### Stats

| Metric | Value |
|---|---|
| Commits | 42 |
| Contributors | 7 |
| Files changed | 106 |
| Insertions | +1,997 |
| Deletions | −882 |
| Issues closed | #82, #92, #101, #102, #104, #107, #108, #124, #189, #193, #230 |
| PRs merged | 35 |

---

### 🙏 Contributor Acknowledgements

This release would not have been possible without the incredible dedication and talent of every single contributor. The OxiCloud community continues to grow, and every contribution — from a one-line fix to a 22-commit marathon — makes this project stronger.

---

#### @BillionClaw — 22 commits ⭐ MVP of this release

An absolutely extraordinary contribution. **BillionClaw** single-handedly tackled the majority of this release, delivering a sweeping wave of fixes that touched every layer of OxiCloud — from **WebDAV quota enforcement** and **folder ownership security**, to **dark mode toggle fixes**, **i18n completeness**, **trash view rendering**, **shared link URLs**, **CI pipeline fixes**, and **ARMv7 compilation support**. The depth and breadth of these contributions is remarkable. Every fix came with clear commit messages, proper issue references, and thoughtful descriptions. BillionClaw didn't just fix bugs — they systematically audited and hardened OxiCloud's core functionality. The download progress bar for >2GB files and the thumbnail memory optimization show a keen eye for performance and user experience. **Thank you, BillionClaw, for this exceptional level of commitment to OxiCloud. You are a pillar of this community.** 🏆

---

#### @jaredwolff — 8 commits

**Jared** continues to be one of OxiCloud's most impactful contributors. This release features his landmark **SQLx migration system** — a foundational infrastructure change that replaces the fragile custom schema loader with proper versioned migrations, ensuring rock-solid database upgrades for every deployment going forward. He also patched the critical **RUSTSEC-2026-0037 security advisory**, fixed the persistent **Nextcloud sync conflict** that plagued desktop and mobile clients, resolved the **photos-view-in-trash CSS leak**, removed the non-portable `target-cpu=native` from Docker builds, and aligned the entire CI pipeline with the new migration system. Jared's contributions consistently tackle the hardest, most impactful problems. **Thank you, Jared, for your continued engineering excellence and for making OxiCloud more reliable and secure with every release.**

---

#### @EdouardVanbelle — 5 commits

**Edouard** brought a beautiful **drag & drop into breadcrumbs** feature that makes file organization feel natural and intuitive. He also added the missing home icon, eliminated duplicate breadcrumb code, improved the developer experience with cache-free dev-mode static assets, and cleaned up code style. Every contribution shows a strong focus on user experience and code quality. **Thank you, Edouard, for bringing polish and elegance to OxiCloud's interface. Your UI contributions make a real difference in how people interact with the platform every day.**

---

#### @nk-designz (Nico Kahlert) — 2 commits

**Nico** opened the door to **enterprise Kubernetes deployment** by creating a complete Helm chart with configurable values, optional WOPI integration, and thorough documentation. This is a game-changer for teams looking to deploy OxiCloud in production Kubernetes clusters. **Thank you, Nico, for bringing OxiCloud to the cloud-native world. This Helm chart makes professional deployment accessible to an entirely new audience.**

---

#### @raenur (Nathan Shepperd) — 2 commits

**Nathan** contributed practical improvements to the **README** and **example.env** with documentation suggestions for accessing OxiCloud remotely after first install — exactly the kind of first-time-user perspective that makes onboarding smoother for everyone. **Thank you, Nathan, for thinking about the new user experience and making the first steps with OxiCloud clearer and more welcoming.**

---

#### @JVMerkle (Julian Merkle) — 1 commit

**Julian** fixed the calendar color validation to support **RGBA colors**, a small but important change that unblocks users who rely on RGBA color specs in their CalDAV clients. **Thank you, Julian, for this targeted and well-crafted fix. CalDAV compatibility improves with every contribution like this.**

---

#### @zjean — 1 commit

**zjean** resolved all remaining **clippy warnings and rustfmt issues** to bring the codebase into full CI compliance — a foundational cleanup that keeps the build green for everyone. **Thank you, zjean, for your dedication to code quality and for ensuring OxiCloud maintains a clean, warning-free codebase.**

---

### New Contributors 🎉

A warm welcome to the contributors making their first contribution to OxiCloud in this release:

* @BillionClaw made their first contribution in https://github.com/DioCrafts/OxiCloud/pull/208
* @EdouardVanbelle made their first contribution in https://github.com/DioCrafts/OxiCloud/pull/235
* @nk-designz made their first contribution in https://github.com/DioCrafts/OxiCloud/pull/198
* @raenur made their first contribution in https://github.com/DioCrafts/OxiCloud/pull/197
* @JVMerkle made their first contribution in https://github.com/DioCrafts/OxiCloud/pull/202

---

**Full Changelog**: https://github.com/DioCrafts/OxiCloud/compare/v0.5.2...v0.5.3
