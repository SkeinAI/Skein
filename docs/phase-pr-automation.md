# Phase PR Automation in Skein — Guidance

This document describes the two Skein workflows and the cron job set up to (1) resolve
`phase-rs/phase` "p1-core-mechanic" issues into pull requests, and (2) monitor your open
PRs for new comments. It also records the related fixes and the local run/setup gotchas
discovered while building this.

> Status: workflows + cron job were inserted directly into `skein-data/skein.db` (the
> Skein app's database). They appear in the Workflow editor after the app refreshes its
> list (see "Make them appear in the UI").

---

## 1. What was created

| Kind | ID | Name | Trigger |
|------|----|------|---------|
| Workflow | `wf_phase_p1_resolver` | **Phase: Resolve P1 Issue → PR** | Manual (▶ in the Workflow editor) |
| Workflow | `wf_phase_pr_monitor` | **Phase: PR Comment Monitor** | Cron, every 30 min |
| Cron job | `cron_phase_pr_monitor` | Phase PR Comment Monitor | `*/30 * * * *` → runs `wf_phase_pr_monitor` |

Your pre-existing `Phase-push` workflow was **not** modified.

### Workflow A — `Phase: Resolve P1 Issue → PR` (manual)

```
start ──▶ pick_issue (agent · Bash) ──▶ implement_ship (agent · Bash) ──▶ answer
```

- **start** — input variable `query` (default = the p1-core-mechanic issues URL). Reachable
  downstream as `${start.query}`.
- **pick_issue** (`tools: [Bash]`, model `claude-sonnet-4-20250514`) — uses `gh` to list open
  `label:priority:p1-core-mechanic` issues in `phase-rs/phase`, filters out any with a
  linked/closing PR (GraphQL `closedByPullRequestsReferences` + cross-reference timeline),
  and picks the lowest-numbered available one. Honors a specific issue # in `${start.query}`.
- **implement_ship** (`tools: [Bash]`) — the heavy step. It does **not** implement in Skein.
  It shells out to the `claude` CLI so the implementation runs in its **native** runtime:
  `cd ~/Dev/dripsmvcp/phase` → `claude -p "/engine-implementer …"` (the real plan→implement→
  review→commit pipeline) → opens a PR to `phase-rs/phase` (**without** `--auto`).
- **answer** — surfaces the result (PR URL or the reason it stopped) into the conversation.

### Workflow B — `Phase: PR Comment Monitor` (cron, 30 min)

```
start ──▶ scan (agent · Bash) ──▶ answer
```

- **scan** (`tools: [Bash]`) — lists your open PRs (`gh search prs --author=@me --state=open`),
  reads/writes a state file at `~/.config/skein/pr-monitor-state.json` (last-seen timestamp per
  PR), collects comments newer than last-seen (excluding your own), and on new comments writes
  a summary **and** fires a desktop popup via `notify-send`.
- **answer** — logs the summary into the cron conversation.

---

## 2. Why it's built this way (key decision)

The `/engine-implementer` skill in `~/Dev/dripsmvcp/phase/.claude/skills/` is a **Claude Code
orchestrator** — it spawns named sub-agents (`engine-implementation-executor`, `general-purpose`)
and calls other slash-skills (`/engine-planner`, `/review-engine-plan`, `/review-impl`). **Skein
cannot run it faithfully**: Skein "skills" are just a tool that pastes markdown into one agent's
context; it has no named-sub-agent or nested-spawn machinery. Importing the skill and letting a
single Skein agent "follow" it would (a) violate `phase/CONTRIBUTING.md` (engine changes *must*
go through the real `/engine-implementer`) and (b) produce low-quality PRs.

**Decision:** Skein is the *conductor*; the actual engine work is delegated to the headless
`claude` CLI where the skill runs natively. `engine-implementer` only commits — PR creation is a
separate step (`ship-commits` skill or `gh pr create`).

---

## 3. Prerequisites for the workflows to actually run

- **Anthropic API key** in Skein → Settings → Model providers → anthropic. The agent nodes use
  it (model `claude-sonnet-4-20250514`). ⚠️ The default seed model id `claude-haiku-4-20250514`
  is invalid — see §6.
- **`gh` CLI** authenticated (currently `dripsmvcp`, with `repo`/`workflow` scopes).
- **`claude` CLI** on PATH (used by `implement_ship`).
- **phase repo** at `~/Dev/dripsmvcp/phase` with `.claude/skills/{engine-implementer,ship-commits}`.
- `notify-send` (GNOME) for the monitor's desktop popups.

---

## 4. How to use

### Make them appear in the UI
The app caches the workflow list, so after install you must refresh:
- **Reliable:** quit Skein and relaunch, then open the Workflow editor.
- **Without restart:** navigate away from the Workflow page and back (re-queries `list_workflows`).

### Run Workflow A (resolve an issue)
Open **Phase: Resolve P1 Issue → PR** and click ▶. Optionally set the `query` input to a specific
issue number/URL; otherwise it auto-picks the lowest-numbered available p1-core-mechanic issue.
It runs in auto-approve (Yolo) mode. The PR is opened **without auto-merge**, so you review it.

### Workflow B (monitor) runs itself
The cron job triggers every 30 minutes. Results appear in its conversation; new comments also
raise a `notify-send` popup. First run creates `~/.config/skein/pr-monitor-state.json`.

---

## 5. Reinstall / modify the workflows

The definitions were inserted by `/tmp/install_phase_workflows.py` (idempotent upsert into the
`workflow` and `cron_job` tables of `skein-data/skein.db`). To change them, edit that script and
re-run it:

```bash
python3 /tmp/install_phase_workflows.py
```

Notes:
- Writes are **additive** and safe while the app runs (DB is WAL); refresh the UI to see changes.
- `config` = the ReactFlow graph `{nodes, edges, metadata}`; `published_config` is set equal to it
  so the workflow is runnable immediately. Variable interpolation uses **`${node.field}`** syntax
  (e.g. `${start.query}`, `${pick_issue.response}`), not `{{ }}`.
- Prefer editing in the Workflow UI for small tweaks; use the script for structural changes or
  re-provisioning on a fresh DB.

---

## 6. Related fix — invalid Anthropic Haiku model id

The Anthropic provider seed shipped a **non-existent** model id `claude-haiku-4-20250514` (the
2025-05-14 launch was Opus 4 / Sonnet 4 only — no Haiku), which 404'd on the connection test and
any chat using it.

- **Fix committed:** branch `fix/anthropic-haiku-model-id` (commit `67a020d`) — replaced it with
  `claude-haiku-4-5-20251001` in `crates/skein-core/src/db/model_providers/anthropic/provider.yaml`.
- **Existing DB caveat:** seeds only apply to a *fresh* DB. Your current `skein.db` still has the
  bad row — fix/delete that model in Settings → Model providers → Anthropic, or start fresh.

---

## 7. Running Skein locally (environment gotchas)

These bit us repeatedly while testing; recorded here for future runs.

- **Launching from the VS Code snap crashes** with `undefined symbol: __libc_pthread_init,
  GLIBC_PRIVATE` (snap injects core20 GTK module paths). Launch with those stripped:
  ```bash
  cd skein-ui
  env -u GTK_PATH -u GTK_EXE_PREFIX -u GTK_IM_MODULE_FILE \
      -u GDK_PIXBUF_MODULE_FILE -u GDK_PIXBUF_MODULEDIR -u GIO_MODULE_DIR \
      -u GSETTINGS_SCHEMA_DIR -u LOCPATH -u SNAP_LIBRARY_PATH -u LD_LIBRARY_PATH \
      npm run tauri dev
  ```
  A normal (non-snap) GNOME Terminal does not need this.
- **Stale `target/` from the old `flock` path** can break the Tauri build (it reads plugin
  permission TOMLs from a dead `…/flock/…` path). Fix: `cargo clean` (or targeted
  `cargo clean -p skein-desktop -p skein-workflow …`).
- **Use the desktop window, not a browser.** Opening `http://localhost:5173` in a browser throws
  `Cannot read properties of undefined (reading 'metadata')` in `getCurrentWindow()` — Tauri APIs
  only exist inside the Skein window.
- A background-launched window opens **behind** your focused window (GNOME focus-stealing
  prevention). Alt-Tab to it, or launch from your own terminal so it gets focus.

---

## 8. Risks & limitations (read before relying on Workflow A)

1. **Bash/agent timeout** — `implement_ship` leans on a single `claude -p` call; a large engine
   fix can exceed the agent node / Bash timeout and fail. v2 idea: run `claude` in the background
   and poll. Run Workflow A **manually** (not via cron — the cron runner caps at ~15 min).
2. **Real PRs** — opened to a live repo. A is manual-trigger and PRs are opened **without
   `--auto`**, so you stay in control. Review before merging.
3. **Fork vs upstream** — your `origin` is the `dripsmvcp` fork; PRs target `phase-rs/phase`
   (`upstream`). The implement step pushes to the fork and opens the PR against upstream.
4. **Cost** — the heavy reasoning runs through the `claude` CLI (your Claude Code subscription);
   Workflow B spends a little Anthropic-API per 30-min run.

---

## 9. ⚠️ Security note

An Anthropic API key was pasted into chat during setup. Treat it as **compromised**: revoke it in
the Anthropic Console and issue a new one. Store keys only in Skein's Settings (encrypted at rest,
machine-bound) — never in chat, files, or commits.
