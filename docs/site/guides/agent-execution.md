# Interactive agent sessions

The Agents views and repository, project and task actions can start Codex or
Cursor sessions on this device. Register an ordinary Git-backed JJ folder, or
link a native repository workspace, then choose its exact starting revision. The
app creates a new isolated JJ workspace. Dirty source files remain in the source
folder. Agents edit the isolated workspace, which stays available for review
after the session ends.

Named setups save an avatar, instructions, runner and model, effort and mode
preferences. After connection, the app shows the runner's advertised choices and
applied preferences. Apply an execution mode before sending a prompt.
Instructions and project/task context accompany the first prompt.

The conversation shows streamed text, tool activity, plans and exact permission
choices. Follow-ups queue in the warm session; Stop turn cancels the selected
turn, and End session closes the adapter. Resume uses the adapter's explicit
load capability. New session with saved context starts a fresh adapter session
and labels that distinction. Neither action sends a prompt automatically.

Request identities and drafts survive refresh and navigation. After a lost
response, Check outcome queries the original request. An unknown outcome keeps
the request unresolved and the workspace available; review it before explicitly
continuing. Server restart never resubmits prompts. Sanitized history publishes
through AgentStore, and the published counter distinguishes durable observations
from activity still awaiting publication.

Review changes opens the retained revision. Explicit cleanup retains a JJ
bookmark and refuses ignored, untracked or special files. It does not merge or
push agent changes.

## Enable execution

Start the LocalTrusted server with `JJ_STORE_ACP_EXECUTABLE` set to the absolute
path of a rebuilt `jj-acp` binary supporting control version 2. Keep the paired
custom `jj` beside the server, or select it with `JJ_STORE_AGENT_JJ_EXECUTABLE`.
Install and authenticate `codex-acp` or `cursor-agent` separately and make it
available on the server's PATH. The app does not install adapters or store their
credentials. Privacy-rule preparation may briefly show Preparing agent sessions
before launch is available.

For desktop resource staging, pass `--acp-executable /absolute/path/to/jj-acp`
to `node scripts/desktop.mjs stage`. The optional executable is bundled with the
paired JJ runtime. Without it, ordinary browsing remains available.

Execution currently admits one warm process at a time, up to sixteen queued
commands, sixty-four retained sessions and sixty-four saved setups. Setups and
lifecycle receipts are private device state outside repository checkouts.
Repository history remains subject to independent code-repository access; access
to a project or task home does not grant execution in linked code.
