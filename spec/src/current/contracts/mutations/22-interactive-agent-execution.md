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
