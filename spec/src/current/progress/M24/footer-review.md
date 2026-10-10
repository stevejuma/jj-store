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
