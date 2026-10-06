# Local Stacks operation descriptions

Local Stacks writes readable action summaries into native operation metadata at
transaction commit. Summaries normalize whitespace and omit command payloads;
Undo/Redo retains its native operation-reference descriptions. Legacy merge
cleanup recognizes both historical and normalized membership descriptions and
still requires native reference-transition and ancestry proof. Existing
operation history is never rewritten to change presentation.
