use super::*;
pub(super) fn event_parent_map(
    graph: &ReviewEventGraphV1,
) -> BTreeMap<ReviewObjectRef, BTreeSet<ReviewObjectRef>> {
    graph
        .nodes
        .iter()
        .map(|node| (node.event, node.parents.iter().copied().collect()))
        .collect()
}
pub(super) fn graph_is_ancestor(
    parents: &BTreeMap<ReviewObjectRef, BTreeSet<ReviewObjectRef>>,
    ancestor: ReviewObjectRef,
    descendant: ReviewObjectRef,
) -> bool {
    let mut pending = vec![descendant];
    let mut seen = BTreeSet::new();
    while let Some(event) = pending.pop() {
        if event == ancestor {
            return true;
        }
        if seen.insert(event) {
            pending.extend(parents.get(&event).into_iter().flatten().copied());
        }
    }
    false
}
