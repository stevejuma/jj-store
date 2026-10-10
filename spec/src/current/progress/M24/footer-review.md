# Conversation footer review

E-CHAT-FOOTER-001 protects the collapsed formatting toolbar from shrinking and
gives expanded formatting a full row above repository/runtime/Send controls. The
reproduced clipping regression passes at narrow and wide widths, including
keyboard access, with live inbox panel geometry verified. M24 remains In
progress; independent workspace qualification, existing resize diagnostics and
unapproved visual baselines remain unchanged.

E-CHAT-FOOTER-002 gives the workspace button a themed secondary resting surface,
retains hover feedback and truncates its label before wrapping. Wrapped rows
align with the formatting toolbar. Three focused interaction/accessibility
stories and live one-row geometry pass. M24 stays In progress with independent
workspace qualification, resize investigation and visual approval unchanged.

E-UI-MENU-002 uses secondary-sized, regular-weight muted headings in footer
menus and shared popovers. Live typography and existing menu interactions retain
their behavior. M24 remains In progress; workspace qualification and visual
baselines remain open.

E-CHAT-MAIN-001 lands the 42-change chat-body stack through 7ea50d09 in the main
workspace without conflicts. The main-built backend and Vite replace the
chat-body processes while retaining the fixture and exact saved configuration.
The refreshed manual records workspace inspection and local file-review limits.
M24 and M13.3 qualification remain unchanged; this handoff is not release or
full-flow acceptance, and visual baselines remain unapproved.

E-CHAT-CARET-001 shares the prototype's editor caret theme with live Thread and
agent document composers. Reading carets stay grey; draft carets use the primary
colour. Both retain 2px strokes and the taller shape, with the same draft
typography. Focused production/prototype parity and view-switching checks retain
the existing editing and Classic contracts. The first-line clipping regression
was reproduced on all three surfaces: a negative caret margin hid part of the
stroke. Zero overhang retains the full shape within editor bounds, including
multiline drafts. Live Thread geometry confirms the fix. M24 remains In
progress; existing resize diagnostics and unapproved visual baselines remain
independent.

E-AGENT-TRANSCRIPT-VIEW-001 applies the saved Document/Classic preference to
Projects landing and project-specific responses, repository sessions, legacy
chunks and embedded captured context. Document hosts omit the permanent Classic
session-review composer. Explicit whole-session ReviewStore forms and recovery
remain available inside the document scroll area; the shared Send footer stays
within the panel. Read-only embedded captures have no prompt input or footer.
Pagination, copy identities, sanitized prompt markup and recorded command/output
content remain available. Live Focus geometry and focused renderer, remount,
interaction/accessibility and projection checks qualify this presentation slice
in the enclosing change umouvlrv. M24 remains In progress. Provider, restart
execution, existing ResizeObserver investigation and visual baselines retain
their independent gates; no visual baseline is approved.

E-CHAT-FOOTER-003 keeps Agent repositories immediately before Agent with the
same compact button treatment, adds distinct themed hover feedback to Codebase,
Workspace and individual Pin/Archive actions, and gives the shared inbox split
control a theme border. The Workspace menu uses its own viewport-bounded width;
both inbox sidebar headers retain smaller matching horizontal insets. Existing
repository selections, audience, draft and device-local pin/archive contracts
remain unchanged in enclosing change pqnxyszz. Focused stories and live geometry
review these presentation changes. M24 remains In progress; existing
ResizeObserver diagnostics and unapproved visual baselines stay open.

E-AGENT-STATE-001 reviews the enclosing default-workspace change xmorxmqv.
Explicit reopen failures settle their receipts, fatal scoped ACP errors retire
live workers, and Stop can cancel an exact turn while Send remains unresolved.
The shared footer, transcript activity placement, centred captions, prose parity
and code-attachment inbox grouping follow the updated owning contracts. M24
stays In progress. Existing adapter/provider, live permissions, desktop,
ResizeObserver and visual baseline gates remain independent and unapproved.
The wider fixture audit reproduces ten existing discussion/reply story failures
at parent 7601e3cf. Its two activity regressions are repaired; reply-interface
story migration and existing architecture oversizes remain separate open work.
