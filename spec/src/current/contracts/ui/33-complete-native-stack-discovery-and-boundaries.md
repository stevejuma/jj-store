# Complete native stack discovery and boundaries

The
[native stack discovery contract](../local-jj/31-complete-native-stack-discovery-and-boundaries.md)
owns inventory, boundaries and membership semantics.

The Stacks picker searches all dependent names and aliases through their
containing top-level entry; ordinary repository branch browsing is unchanged. A
top-level entry with only a lower member applied shows **Partially applied**;
Workspace keeps the actual applied slice until the user applies the upper head.
Cards show newest branches first, with compact Branches headers and existing
Workspace controls. Shared lower cards carry **Shared**. Their original raw
bookmark identities continue to own drafts, selections and adjacent diffs, with
only one visible diff column per owner. Native boundaries remain usable before
history pages arrive; unloaded card content uses skeletons. Drawing-only missing
or indirect edges cannot invalidate a proven native boundary.
