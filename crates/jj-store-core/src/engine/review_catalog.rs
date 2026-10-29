use super::*;
pub(super) fn catalog_root(
    root: &ReviewRootV1,
    catalog: ReviewCatalogKind,
) -> Option<ReviewObjectRef> {
    match catalog {
        ReviewCatalogKind::Threads => root.threads,
        ReviewCatalogKind::Comments => root.comments,
        ReviewCatalogKind::ThreadEvents => root.thread_events,
        ReviewCatalogKind::AnchorRelations => root.anchor_relations,
        ReviewCatalogKind::ArtifactRelations => root.artifact_relations,
        ReviewCatalogKind::Statuses => root.statuses,
    }
}
pub(super) fn set_catalog_root(
    root: &mut ReviewRootV1,
    catalog: ReviewCatalogKind,
    value: Option<ReviewObjectRef>,
) {
    match catalog {
        ReviewCatalogKind::Threads => root.threads = value,
        ReviewCatalogKind::Comments => root.comments = value,
        ReviewCatalogKind::ThreadEvents => root.thread_events = value,
        ReviewCatalogKind::AnchorRelations => root.anchor_relations = value,
        ReviewCatalogKind::ArtifactRelations => root.artifact_relations = value,
        ReviewCatalogKind::Statuses => root.statuses = value,
    }
}
