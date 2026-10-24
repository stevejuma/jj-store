# Planned thread file storage boundary

The
[M35 implementation plan](../../../../../docs/maintenance/thread-files-implementation-plan.md)
and
[owning collaboration contract](../collaboration/45-thread-files-and-attachments.md)
specify future thread-owned files under JST-COLLAB-037 through JST-COLLAB-039.
The implementation plan index links this work without changing existing runtime
architecture or promoting any gate.

JST-ARCH-002 retains independent storage/model/WAL authority. File metadata and
retained content use existing native/local publication providers; browser
previews, agent scratch copies and generated indexes cannot become authority.
Content stays outside eager collaboration metadata materialization. The first
implementation slice must prove complete retained closures, bounded content
reads and compatibility before dependent API/UI work.

This planning record adds no persistence codec, store interface, native
manifest, companion format, remote-transfer profile or execution host behavior.
All M35 stages remain Planned; existing storage identity and origin gates remain
in force.
