# Interactive agent execution

JST-COLLAB-031 through JST-COLLAB-033 add owned service contracts for agent
setup, targets, explicit launches, prompts, current permission decisions,
lifecycle commands and stable request outcomes. Durable commands survive browser
disconnect. Receipts distinguish admitted, completed, refused and unknown states
without re-dispatch. JJ providers and AgentStore publication remain separate
owned interfaces.

JST-MUT-001 remains unchanged by the read-only managed capture/workspace status
projections. Identity joins and repository navigation neither create a command
intent nor start, resume, configure or prompt an agent. Ordinary discussion
continues to use exact native session/message targets. Original mutation
requests, authorization, conditional setup revisions, unknown outcomes and
receipt recovery retain their existing formats and behavior.

Explicit runtime installation is an installation-owned side effect, never a
repository mutation. Its original intent is privately journaled before npm
starts. Duplicate IDs return the retained receipt; changed intents conflict.
Installations serialize and refuse while an agent process is open. Unselected
staging directories never participate in discovery. Successful verification,
selection and completed receipt share one atomic journal write. A service
restart marks unselected interrupted installs as interrupted without replay;
only a new explicit command may prepare another directory. Unknown browser
outcomes query the original receipt and never resend installation automatically.
