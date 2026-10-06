# Usable capture preview scope

The current capture delivery sequence integrates with verified committed main,
then prioritizes the existing Capture settings and Agents UI and a runnable
LocalTrusted Codex preview. It reuses current AgentStore, native directory and
Write admission, bounded workers and receipt-backed manifest publication. The
full multi-harness/conversation/provenance/review/archive roadmap remains
separately deferred. This changes execution scope and acceptance sequencing,
without introducing another authority, runtime or legacy storage profile.

Agent capture management has a dedicated installation-scoped Agents settings
section. It composes the owned shared Settings rows and foundational controls;
repository navigation and the capture observation lifecycle remain independent.
Capture retains startup-resolved harness directories beside the cached Copy
observation, so admitted settings reads expose configuration without directory
inspection. The four Agents settings tabs share one mounted observation owner
and retain request drafts and exact pending commands across tab changes.
