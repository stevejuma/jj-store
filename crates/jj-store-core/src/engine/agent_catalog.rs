use super::*;
pub(super) fn catalog_root(
    root: &AgentRootV1,
    catalog: AgentCatalogKind,
) -> Option<AgentObjectRef> {
    match catalog {
        AgentCatalogKind::Sessions => root.sessions,
        AgentCatalogKind::Aliases => root.aliases,
        AgentCatalogKind::Checkpoints => root.checkpoints,
        AgentCatalogKind::Associations => root.associations,
        AgentCatalogKind::Relations => root.relations,
        AgentCatalogKind::Sources => root.sources,
        AgentCatalogKind::Projections => root.projections,
    }
}
pub(super) fn set_catalog_root(
    root: &mut AgentRootV1,
    catalog: AgentCatalogKind,
    value: Option<AgentObjectRef>,
) {
    match catalog {
        AgentCatalogKind::Sessions => root.sessions = value,
        AgentCatalogKind::Aliases => root.aliases = value,
        AgentCatalogKind::Checkpoints => root.checkpoints = value,
        AgentCatalogKind::Associations => root.associations = value,
        AgentCatalogKind::Relations => root.relations = value,
        AgentCatalogKind::Sources => root.sources = value,
        AgentCatalogKind::Projections => root.projections = value,
    }
}
