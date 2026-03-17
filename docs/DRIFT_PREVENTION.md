# Drift Prevention — Genesis Protocol

Standing rules to prevent numerical, URL, DOI, and version drift across public surfaces.

---

## The Short Prompt

Run before any public-facing action:

> Verify: build clean, tests 403, crate stub constants match, README numbers match docs/CANONICAL_COUNTS.md, DOI is 18729652, engine version is 0.1.0, crate version is 0.1.1, commit matches HEAD, repo URL is FTHTrading/Genesis, sensitivity hashes still summarized only. Flag any drift.

---

## Update Sequence

When any canonical number changes, update files in this order:

1. `docs/CANONICAL_COUNTS.md` (source of truth — always first)
2. `README.md`
3. `CITATION.cff`
4. `.zenodo.json`
5. `publish/genesis-multiverse/src/lib.rs` (constants)
6. `publish/genesis-multiverse/Cargo.toml` (if version changes)
7. `publish/genesis-multiverse/README.md`
8. `REPLICATION_LEADERBOARD.md`
9. `COLLAPSE_DEFINITION.md`
10. `moltbook/post_current_canon.md`
11. All other `moltbook/*.md` files
12. Live Moltbook platform (manual)

---

## Drift Detection Grep

Run from repo root after any change:

```powershell
# Check for old URL
Get-ChildItem -Recurse -Include *.md,*.rs,*.toml,*.json |
  Where-Object { $_.FullName -notmatch 'target' } |
  Select-String -Pattern 'FTHTrading/AI|cd AI|18646886'

# Should return zero matches outside audit files.
```

---

## Version Bump Checklist

Before publishing any new version:

- [ ] `Cargo.toml` (workspace root) version updated
- [ ] `publish/genesis-multiverse/Cargo.toml` version updated
- [ ] `publish/genesis-multiverse/src/lib.rs` VERSION constant matches
- [ ] `CITATION.cff` version field updated
- [ ] `.zenodo.json` version field updated
- [ ] `docs/CANONICAL_COUNTS.md` version identifiers table updated
- [ ] `moltbook/post_current_canon.md` engine table updated
- [ ] `cargo publish --dry-run` succeeds
- [ ] `cargo publish` executed
- [ ] docs.rs page verified after publish

---

## Post-Test-Change Checklist

When test count changes (new tests added, tests removed, ignored status changes):

- [ ] Update `docs/CANONICAL_COUNTS.md` test counts table
- [ ] Update `publish/genesis-multiverse/src/lib.rs` ENGINE_TESTS constant
- [ ] Update `README.md` test references (3 locations)
- [ ] Update `.zenodo.json` description
- [ ] Update `CITATION.cff` abstract
- [ ] Grep all `moltbook/*.md` for old test count
- [ ] Update `publish/genesis-multiverse/README.md`

---

## Post-Experiment-Change Checklist

When experiment corpus changes (new experiments, new seasons):

- [ ] Update `replication_status.json` header and individual entries
- [ ] Update `docs/CANONICAL_COUNTS.md` tier 3 table and hash registry table
- [ ] Update all files in the version bump checklist above
- [ ] Update `REPLICATION_LEADERBOARD.md` experiment count and hash tables
- [ ] Update `COLLAPSE_DEFINITION.md` verification section counts
- [ ] Grep for old experiment/world counts across all `.md` files

---

## Files That Are Intentionally Historical

Do NOT "fix" numbers in these files — they document a frozen point in time:

| File | Why |
|---|---|
| `papers/genesis_protocol_paper.md` | Covers S1+S2 scope (38/5,680). Correct for stated scope. |
| `IP_RECORD.md` | Frozen bundle at 2026-02-22 (345 tests, 8 experiments, 680+ worlds). |
| `WHITEPAPER.md` appendix | Early development phases (67 tests, 580 worlds). Historical. |
| `GENESIS_ECOSYSTEM_AUDIT.md` | Audit record with historical references. |
| `MOLTBOOK_AUDIT.md` | Audit record with historical references. |
