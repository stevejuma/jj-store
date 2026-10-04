# UI presentation guidance

Read `spec/src/ui.md` and inspect the existing Settings forms before adding or
changing a form. Settings is the default presentation for all form-like
surfaces, including dialogs and repository creation/conversion.

- Group related fields in a bordered, rounded
  `Field.Group class="community-settings__option-group"`.
- Use `StoreWorkField` for a label and optional description on the left, with
  the input or control on the right. Keep its default bordered input treatment;
  reserve borderless controls for an explicitly read-only presentation.
- Use the existing `store-settings__action-row` for settings commands:
  description on the left, buttons on the right, with the shared border and
  responsive layout.
- Separate form groups and contextual notices with the shared form spacing.
  Preserve field labels, validation associations, keyboard focus and narrow
  layouts.
- Use `StoreNotice` for transient request errors, with a stable resource key and
  condition lifetime for retryable reads. Keep validation and exact recovery
  beside their controls; do not repeat the same error inline and in a toast.

Reuse repository-owned Community compositions and public Design Core primitives.
Do not introduce an unstyled collection of labels, inputs or standalone buttons
when an established settings pattern covers the interaction.
