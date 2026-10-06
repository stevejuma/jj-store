---
schema: 1
id: E-M20-002
---

# E-M20-002 details 9

<!-- evidence-field:ui_acceptance markdown -->
## Ui acceptance

65/65 focused controller/history units (controller code unchanged since that run) and 113/113 complete directly affected interaction/accessibility stories in 22 files passed. Web and catalog checks have zero errors/warnings; 214 catalog and 1016 architecture modules passed; UI ownership passed. Exact final regression group 9/9 passed, measuring trailing chips, full-width title separator, 36px input and balanced scroll reserve on stack/diff columns in both tabs while the Branches picker retains 0px reserve. Footer themes preserve drafts and show the Pencil edit icon with matching very light surfaces. Browser proof: .local/stack-header-drops-evidence/workspace-refinements.jpg and branches-refinements.jpg. First 435 historical evidence objects remain byte-identical.
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte
<!-- /evidence-field -->

<!-- evidence-field:story_scenarios list -->
## Story scenarios

- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte#Content
  padding separates components from edge scrollbars
- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte#Stack
  columns omit redundant application and base metadata
- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte#Kebab
  menus have distinct hover and aligned branch controls
- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte#Dark
  kebab menus have distinct hover and aligned branch controls
- catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte#Assign
  selected changes with settings rows and branch command picker
- catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte#Create
  independent stack
- catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte#Create
  dependent stack with command picker
- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte#Applied
  branches have their own group
- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte#Apply
  and unapply actions occupy the stack body
- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte#Workspace
  target branch appears above body tabs
- catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte#Retained
  draft matches the light footer chips
- catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte#Retained
  draft matches the dark footer chips
- catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte#Workspace
  chip sits at the title row edge
<!-- /evidence-field -->

<!-- evidence-field:source_acceptance markdown -->
## Source acceptance

Design Core dark count contrast corrected upstream and committed d7ebf3e1 after complete required nonvisual checks. Current ignored foundational validation setup hardlinks byte-identical source tokens; tracked package manifest and lockfile unchanged. Registry-resolved release qualification remains separately pending.
<!-- /evidence-field -->

