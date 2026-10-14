# Shell composition

The shell defaults to Branches, a collapsed left sidebar and closed 320px right
sidebar. Per-instance history, selection and controllers never persist across
stories. Repository compact popovers retain focus restoration; mobile navigation
uses the same public shell stages. Community-only room actions are omitted.

Use one shared Application menu for native repository actions and supported
saved-settings reload. Desktop places it at the top navigation's right edge;
limited browser viewports hide that navigation and place the menu at the right
of the body toolbar, alongside the existing navigation controls. Workflows uses
the same mobile body-header composition as Projects, including loading and
History/detail views. Settings and other hubs retain access to the same menu.
Viewport changes preserve open dialogs, drafts, original requests and focus
restoration; moving the trigger must not remount its command owners.
