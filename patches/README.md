# Temporary Bits UI repair

The user authorized this reproducible pnpm patch on October 31, 2026. It targets
released `bits-ui@2.20.2`; `manifest.json` records the registry identity, exact
patch SHA-256, and original/patched file hashes.

The patch contains three retained owner corrections:

- Link Preview owns and cancels its deferred pointer-selection probe and avoids
  reads after destruction. The listener-safe cleanup is already in npm 2.20.2.
  Provenance: Plugins `bf492db552702fd310dcc6439152d42d472de068`,
  `spec/records/2026-10-11-plugin-test-unblock.md`; Notes' prior 2.20.2 patch
  has SHA-256
  `d9b49a52c343dfbd392e235b5d748e5922f3e16e727cf92064437a95c90530f9`.
- Slider's single/multi pointer-down, move and up handlers reject work after
  destruction. Provenance: Plugins `201b1feb60fe97ee383ea30b061e453e1ad5e853`,
  `spec/records/2026-10-27-preview-lifecycle-slider-drain.md` (LP-SPEC-058).
- FocusScope preserves focus and autofocus callback counts when mounted again
  with its existing container. Real unmount retains cleanup and trigger focus.
  Provenance: owner `692fbdf5095941464561189fd81ffc2cb4b81c6c`,
  <https://github.com/huntabyte/bits-ui/pull/2264>.

## Consumer installation

pnpm patches belong to the installation root. Installing published Design Core
does not apply its patch in a downstream project. Each consumer must copy the
identical `bits-ui@2.20.2.patch`, verify its SHA-256 against `manifest.json`,
and declare this entry in its own root package-manager configuration:

```json
{
  "pnpm": {
    "patchedDependencies": {
      "bits-ui@2.20.2": "patches/bits-ui@2.20.2.patch"
    }
  }
}
```

Resolve Bits UI 2.20.2, regenerate and commit that root's lockfile with pnpm,
then use frozen installs. Root patch hash:
`debd3bc7d43917f0967f0cc1e212fc5c7721c751d3b92bf8f31030d1c96289ad`. The public
package ships this patch and manifest for explicit consumer intake. No install
hook edits a consumer's files. An unpatched npm install does not qualify the
patched runtime acceptance contract.

## Removal

Remove this patch only after the resolved released dependency contains all three
equivalent fixes. Update every installation root, rerun exact teardown/focus
regressions and complete functional/accessibility/public-package acceptance, and
record the released source and consumer receipts. A version bump or merged
upstream PR alone does not satisfy removal. Preserve earlier red receipts.
