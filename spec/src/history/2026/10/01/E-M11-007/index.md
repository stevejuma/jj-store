---
schema: 1
id: E-M11-007
---

# E-M11-007

<!-- evidence-field:id markdown -->
## Id

E-M11-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-development-project-task-seed
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change vqqlmmol.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main clean at zllovtln / bf5c6bfa. Interactive fixture has no projects/tasks. Fresh repository setup seeds code, agent activity and reviews only. Existing development fixture tests pass 8/8; the new exact work-seed target fails once because its seeder module is absent.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- package.json
- scripts/dev-demo.mjs
- scripts/dev-demo-work.mjs
- scripts/dev-demo-work-samples.mjs
- scripts/dev-demo-work.test.mjs
- web/tests/real/work-seed.spec.ts
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/api.md
- spec/src/spec-governance.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dev-demo-work.test.mjs#work seed adds linked projects, all task
  states, subtasks and threaded discussion once
- scripts/dev-demo-work.test.mjs#work seed preserves edited records, retired
  links and comment tombstones
- scripts/dev-demo-work.test.mjs#work seed stops on an unknown outcome and
  resumes from committed records
- scripts/dev-demo-work.test.mjs#work seed resumes an interrupted task creation
  before its initial state update
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M11-007 details 2](details-02.md)
- [E-M11-007 details 3](details-03.md)
