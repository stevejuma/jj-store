# Fresh native comparison outcomes

[Capture evidence](index.md)

The cold fresh run completed 933 Playwright cases. The result sidecars report 911 passed, four changed within tolerance and 18 mismatches. The strict CLI exits 1. Playwright completion alone does not mean visual acceptance. All baseline hashes remain unchanged.

The table lists every non-identical outcome. Existing policy accepts changed-within-tolerance; mismatch remains open. Human visual review is separate.

Inspected AgentChatRuntime and WorkflowRun baseline/actual image pairs show freshly generated request IDs in existing fixture debug output. This explains those sampled differences; the remaining stories need separate stability review. No masking, threshold change, baseline overwrite or source fix is made here.

| Story | Outcome | Difference percent | Dimensions changed |
| --- | --- | --- | --- |
| `jj-store-agent-chat-runtime--configured-agent-enables-first-send-after-confirmed-runtime-configuration` | mismatch | 0.8151 | No |
| `jj-store-agent-chat-runtime--failed-resume-uses-composer-error-and-retains-the-draft` | mismatch | 0.6837 | No |
| `jj-store-agent-chat-runtime--navigation-during-runtime-preparation-retains-the-draft-and-sends-no-prompt` | mismatch | 0.6563 | No |
| `jj-store-agent-chat-runtime--send-prepares-an-ended-session-automatically-and-dispatches-once` | mismatch | 0.7265 | No |
| `jj-store-agent-chat-runtime--unknown-configuration-outcome-retains-the-exact-request-and-never-prompts` | mismatch | 0.6767 | No |
| `jj-store-agent-completed-changes--reconnect-warning-stays-inside-the-composer-and-clears-after-refresh` | changed-within-tolerance | 0.0228 | No |
| `jj-store-agent-completed-changes--saved-changes-warning-offers-a-styled-retry-without-dispatching` | changed-within-tolerance | 0.0176 | No |
| `jj-store-agent-session-settings--chat-header-menu-and-list-toggle-retain-the-draft` | mismatch | 0.5666 | No |
| `jj-store-agent-session-settings--ended-session-groups-review-and-destructive-cleanup-after-titled-runtime-sections` | mismatch | 0.6915 | No |
| `jj-store-agent-session-settings--runtime-settings-submit-exact-negotiated-preferences-without-sending-the-chat-draft` | mismatch | 0.5642 | No |
| `jj-store-agent-session-state--starting-is-directly-after-messages-and-centred-on-its-avatar` | mismatch | 0.4643 | No |
| `jj-store-agent-session-state--working-follows-messages-and-drafted-content-selects-queue` | mismatch | 0.5465 | No |
| `jj-store-agent-transcript-settings--document-capture-actions-expose-native-copy-identities` | mismatch | 0.4405 | Yes |
| `jj-store-interactive-agent-execution--community-avatar-and-harness-pickers` | changed-within-tolerance | 0.0527 | No |
| `jj-store-interactive-agent-execution--exact-permission-choices` | mismatch | 0.6452 | No |
| `jj-store-interactive-agent-sessions--card-start-warms-the-exact-agent-without-opening-a-session` | mismatch | 0.1954 | No |
| `jj-store-projects-hub--create-project-selects-writable-home-and-retains-request` | mismatch | 0.1028 | No |
| `jj-store-projects-hub--task-edit-and-retained-unknown-request` | mismatch | 0.1318 | No |
| `jj-store-reusable-agent-chat--contextual-sidebar-and-persistent-agent-tags` | changed-within-tolerance | 0.0435 | No |
| `jj-store-stacks-topology--native-lanes-keep-dependent-aliases-together-and-wait-for-complete-graph-pages` | mismatch | 0.0859 | No |
| `jj-store-workflow-run--settings-source-pickers-pin-selection-and-clear-dependent-values` | mismatch | 4.3259 | No |
| `jj-store-workflow-run--unknown-run-keeps-exact-request-and-readonly-source` | mismatch | 4.8625 | No |
