# Application story parity

JST-UI-001, JST-UI-002, JST-UI-005 and JST-UI-010 require application stories to
mount the same production application root as the browser. The host owns
transport creation and lifetime; the shared root owns appearance, connection
presentation, notifications, retained workspaces and actual repository/page
composition. Catalog adapters supply deterministic controllers, state and
callbacks. Production cannot import catalog fixtures.

Every story declares application, component or prototype classification.
Application stories cannot substitute shell chrome, placeholder page content,
layout dimensions or alternate rendering branches. Responsive application
coverage uses actual viewport sizes and production preferences. Component tests
identify their production subject; prototypes identify their governing
requirement and purpose. Neither qualifies full application parity.

The repository-owned storybook-parity spec-validator plugin checks rendered
composition and classification, including aliases, re-exports, conditional
branches and preview setup. An import alone is not evidence of rendering.
Unresolved application composition fails with an actionable diagnostic. Browser
acceptance verifies shared geometry, navigation, state and lifecycle; static
validation does not establish visual approval. Existing story identities and
behavior assertions are preserved. Changed stories remain visual-pending.

The Stacks refinement family awaits the declared production Stacks loader before
querying loaded content. It preserves the production root, geometry assertions
and diagnostic guards; real cold application and static-catalog acceptance
remain separate qualification lanes.

The shared root fills the available host width, including flex hosts, while
retaining production scroll owners. Fixture adapters preserve live controller
getters and late-discovered workspace bindings rather than snapshotting them.

Synthetic fixture actions use the reusable padded, dotted-border FixtureControls
panel. Application panels float outside the production layout; component panels
remain inline. Product captures hide these panels while fixture transitions stay
available to deterministic automation. Mobile scenarios declare registered
viewport sizes and exercise the actual responsive application.

Responsive repository and Stacks tabs skip duplicate width/section measurements
so a visible overflow trigger remains mounted and usable during initial layout.

E-LOCAL-TEST-AUDIT-001 records the user-authorized temporary quarantine of two
intermittent story cases under JST-UI-001. The catalog quarantine register lists
their exact identities, evidence, owners and re-enable gates. The Vitest addon
reports the `quarantined` cases as skipped; every enabled interaction and
accessibility check remains blocking. Story composition, play assertions and
runtime diagnostics remain intact. Skipped coverage does not qualify the
original full-suite or release gates, and visual approval remains separate.
