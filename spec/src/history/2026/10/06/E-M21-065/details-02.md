---
schema: 1
id: E-M21-065
---

# E-M21-065 details 2

<!-- evidence-field:out_of_scope list -->
## Out of scope

- Rewriting historical operation metadata, source/sample graph repair or
  mutations, unrelated Rust/UI suites, screenshot comparisons/baseline updates
  and package publication.
- Web/catalog type checks and UI ownership are not rerun because no UI
  production or story source changed; the user-requested graph check reuses the
  already committed stories. Direct live browser inspection remains unavailable
  (CUA returned no accessible tab).
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

New local Stacks operations use readable text. Continue UI verification on main Storybook port 6057 and the retained Vite port 7476 service.
<!-- /evidence-field -->

