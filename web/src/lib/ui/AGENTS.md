# UI presentation guidance

Read the owning current chapter under `spec/src/current/contracts/ui` before
changing presentation. The governing settings/form convention is
`spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md`
(JST-UI-016); navigation and lifecycle are in
`spec/src/current/contracts/ui/09-transient-store-settings.md`. Inspect existing
Settings forms first. These conventions govern new and refreshed forms across
the app, including dialogs and repository creation/conversion. Repository
Settings adopts them now; do not silently restyle legacy global Settings.

## Layout and content

- Use a full-width, single-column layout with vertically stacked, subtly
  bordered cards, moderate radii, theme tokens, restrained typography and
  minimal decoration. Avoid sidebars, multiple content columns, oversized
  padding and nested card borders.
- Compose owned `StoreSettingsCard`, `StoreSettingsHeading`,
  `StoreSettingsFooter`, `StoreReadOnlyRow` and `StoreSettingsIcon` with
  released Design Core primitives. Use appropriate Lucide icons with subtle
  backgrounds.
- Give cards clear headings, optional icons, short descriptions and subtly
  tinted headers. Headers meet the first row with no extra gap. Dividers span
  the entire card width. Keep row spacing consistent, including the first row.
- Separate read-only status, editable configuration and maintenance commands.
  Use compact key-value rows for facts; consistently align icons, labels, values
  and compact Copy/Open controls vertically at the centre.
- Wrap descriptive values, paths and capability lists. Copyable identifiers stay
  on one line with ellipsis, expose their full values on hover and copy exactly.
  Preserve independent Open actions, with Copy at the far right and no reserved
  empty Open slot.
- Use `Field.Group` and `Field.Field` (or the existing `StoreWorkField`
  composition) for editable rows. Retain bordered editable inputs. Place
  label/help left and controls right; keep numeric widths appropriate. Stack at
  the existing mobile breakpoint, let controls shrink, and wrap footer buttons
  without horizontal overflow.

## Numeric fields and joined controls

- Use `StoreNumericSettingRow` for numerical form values. Use
  `StoreNumericSettingNumberRow` when adapting existing numeric configuration
  drafts. Keep each field's established range, canonical unit and validation; do
  not introduce new settings or infer defaults.
- Join the numeric input, unit selector and optional icon-only Reset into one
  control. Show Reset only when the draft differs from its authoritative default
  (including invalid or empty drafts), and hide it again after reset. Reset is
  the last segment after the units, with an accessible label and hover hint.
  Return focus to the input when Reset disappears. Fixed count units are text,
  not a redundant dropdown.
- Offer meaningful time and binary size units. Unit selection changes display
  only: retain exact canonical values and do not dirty a form. Reject fractional
  canonical integers, empty, unsafe and out-of-range entries without rounding or
  clamping. Show exact base values for conversions; label approximations.
  Adapters must preserve invalid drafts and prevent saving them from another
  tab.
- Show authoritative defaults beside field descriptions, separately from
  saved/running status. Omit redundant "Using default value" and "Changed from
  default" labels below inputs; retain the exact-value helper for conversions.
  Reset edits only that field's draft. Save/Apply retain their existing
  semantics. Sliders are optional and use the same canonical range; include an
  accessible label and endpoints, including zero/off meaning.
- Keep the whole joined control's outer border visible, including the right
  edge. Give focus one theme ring around the entire group with a neutral border,
  no extra inner rings and no highlighted border. This also applies to joined
  path/Browse controls. Units and Reset have subtle theme-based hover states;
  disabled controls retain subdued styling and do not gain hover emphasis.
- Preserve keyboard entry, dropdown navigation, reset and optional slider
  controls. Associate help/errors with the input; use `aria-invalid` and field
  invalid/disabled state. Support read-only, disabled, regular and compact rows.

## Actions, navigation and lifecycle

- Commands belong in dedicated bottom section footers. Optional explanation is
  left; buttons align right, primary action last, secondary actions immediately
  before it. Never put commands in ordinary key-value rows or float them under
  descriptions. Copy/Open, Browse and per-field Reset stay attached to their
  property controls.
- Keep maintenance actions separate from configuration Save/Cancel. A form
  spanning several sections has one shared save footer. Keep button order,
  disabled/loading states, validation, permissions and confirmation flows.
- Retain primary pill tabs. Only real subtabs get the compact icon-and-underline
  strip, left-aligned with body padding; no duplicate heading/description strip.
  Active labels are bold with a 3px theme-accent underline over the bottom
  divider. Keep primary/subtab navigation outside the bounded Design Core Scroll
  Area, reduce intervening space, and preserve keyboard/More overflow.
- Preserve routes/history, exact Back and Details restoration, controller
  owners, drafts, pending request identities, stale-response rejection and
  stable automatic refresh. Presentation changes cannot change API/configuration
  formats or authorization. Keep global presentation opt-in for later adoption.
- Use `StoreNotice` for transient request errors with stable resource identity
  and appropriate lifetime. Keep field validation and exact recovery beside
  controls; never duplicate the same error inline and in a toast. Status must be
  understandable without colour alone.

Reuse repository-owned Community compositions and public Design Core primitives.
Use scoped application CSS for the user-directed settings presentation; preserve
released dependencies and do not add local package overrides. Validate the
running app before widening visual test work. Use focused interaction,
keyboard/accessibility, copy and responsive checks; never update visual
baselines without explicit approval.
