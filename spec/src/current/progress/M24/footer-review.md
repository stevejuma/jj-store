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
