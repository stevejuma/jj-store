use super::*;
use jj_lib::protos::local_working_copy as p;
use prost::{
    Message as _,
    encoding::{DecodeContext, WireType, decode_key, decode_varint, skip_field},
};
// Preflight repeated collections before Prost allocates them. Never implement a new codec.
fn fields(
    mut bytes: &[u8],
    allowed: &[u32],
    mut visit: impl FnMut(u32, &[u8]) -> Result<()>,
) -> Result<()> {
    while !bytes.is_empty() {
        let (tag, wire) = decode_key(&mut bytes).map_err(|_| Error::Invalid)?;
        if !allowed.contains(&tag) {
            return Err(Error::Invalid);
        }
        if wire == WireType::LengthDelimited {
            let n: usize = decode_varint(&mut bytes)
                .map_err(|_| Error::Invalid)?
                .try_into()
                .map_err(|_| Error::Limit)?;
            if n > bytes.len() {
                return Err(Error::Invalid);
            }
            let (value, tail) = bytes.split_at(n);
            visit(tag, value)?;
            bytes = tail;
        } else {
            skip_field(wire, tag, &mut bytes, DecodeContext::default())
                .map_err(|_| Error::Invalid)?;
        }
    }
    Ok(())
}
pub(crate) fn checkout(checkout: &[u8], tree: &[u8]) -> Result<Checkout> {
    fields(checkout, &[2, 3], |_, _| Ok(()))?;
    let mut counts = [0usize; 8];
    let mut sparse_count = 0usize;
    fields(tree, &[1, 2, 3, 4, 5, 6, 7], |tag, value| {
        counts[tag as usize] += 1;
        if tag == 2 {
            if counts[2] > MAX_ENTRIES {
                return Err(Error::Limit);
            }
            fields(value, &[1, 2], |tag, state| {
                if tag == 2 {
                    fields(state, &[1, 2, 3, 5], |tag, _| {
                        if tag == 5 {
                            Err(Error::Invalid)
                        } else {
                            Ok(())
                        }
                    })?;
                }
                Ok(())
            })?;
        } else if tag == 3 {
            fields(value, &[1], |_, _| {
                sparse_count += 1;
                if sparse_count > 512 {
                    Err(Error::Limit)
                } else {
                    Ok(())
                }
            })?;
        } else if matches!(tag, 5 | 7) && counts[tag as usize] > 65 {
            return Err(Error::Limit);
        } else if tag == 4 {
            fields(value, &[1, 2], |_, _| Ok(()))?;
        }
        Ok(())
    })?;
    let checkout = p::Checkout::decode(checkout).map_err(|_| Error::Invalid)?;
    let tree = p::TreeState::decode(tree).map_err(|_| Error::Invalid)?;
    let operation =
        m::OperationId::from_slice(&checkout.operation_id).map_err(|_| Error::Invalid)?;
    let workspace = if checkout.workspace_name.is_empty() {
        "default".into()
    } else {
        checkout.workspace_name
    };
    if workspace.len() > 256 || workspace.chars().any(char::is_control) {
        return Err(Error::Invalid);
    }
    let mut paths = BTreeSet::new();
    for entry in tree.file_states {
        safe_path(&entry.path)?;
        if !paths.insert(entry.path) {
            return Err(Error::Invalid);
        }
        let state = entry.state.ok_or(Error::Invalid)?;
        if !matches!(state.file_type, 0..=2) || state.materialized_conflict_data.is_some() {
            return Err(Error::Invalid);
        }
    }
    #[expect(deprecated)]
    let trees = if tree.tree_ids.is_empty() {
        vec![tree.legacy_tree_id]
    } else {
        tree.tree_ids
    };
    if trees.is_empty() || trees.len() > 65 || trees.len().is_multiple_of(2) {
        return Err(Error::Invalid);
    }
    let trees = trees
        .iter()
        .map(|id| m::GitSha1Id::from_slice(id).map_err(|_| Error::Invalid))
        .collect::<Result<_>>()?;
    let sparse = tree
        .sparse_patterns
        .map_or_else(|| vec![String::new()], |p| p.prefixes);
    SparseRules::new(sparse.clone())?;
    if tree
        .conflict_labels
        .iter()
        .any(|label| label.len() > 4096 || label.chars().any(char::is_control))
    {
        return Err(Error::Invalid);
    }
    Ok(Checkout {
        operation,
        workspace,
        trees,
        labels: tree.conflict_labels,
        sparse,
    })
}
