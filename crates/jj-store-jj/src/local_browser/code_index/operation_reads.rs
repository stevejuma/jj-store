//! Explicit basic operation reads; never fall back to a full source repository.
use super::*;

pub(super) async fn read(
    mut context: Context,
    anchor: m::OperationId,
    request: Request,
) -> Result<Response> {
    let (selected, requested_parent) = match &request {
        Request::OperationDiff(query) => (query.operation, query.parent),
        Request::OperationChanges(query) => (query.operation, Some(query.parent)),
        _ => return context.dispatch(anchor, request, true).await,
    };
    if selected != anchor {
        references::selection::validate(
            context.access.binding.clone(),
            anchor,
            selected,
            context.budget.clone(),
            context.limits,
        )
        .await?;
        context = Context::open(
            context.access.binding.clone(),
            selected,
            context.budget.clone(),
            context.limits,
            true,
            vec![],
            false,
        )
        .await?;
    }
    let operation = context.operation(selected).await?;
    let parent = match requested_parent {
        Some(parent) if !operation.parents.contains(&parent) => return Err(Error::Invalid),
        Some(parent) => Some(parent),
        None if operation.parents.len() == 1 => Some(operation.parents[0]),
        None => None,
    };
    if let Some(parent) = parent {
        let loader = loader(context.access.binding.metadata_path())?;
        bounded_op_file(
            context.access.binding.metadata_path(),
            "operations",
            &parent.to_string(),
            &context.budget,
        )?;
        let source = context
            .budget
            .until(loader.load_operation(&o::OperationId::from_bytes(parent.as_bytes())))
            .await?
            .map_err(|_| Error::Incomplete)?;
        let data =
            dto::Operation::try_from(source.store_operation()).map_err(|_| Error::Corrupt)?;
        context.budget.metadata(&data)?;
        bounded_op_file(
            context.access.binding.metadata_path(),
            "views",
            &data.view_id.to_string(),
            &context.budget,
        )?;
        let view = context
            .budget
            .until(source.view())
            .await?
            .map_err(|_| Error::Incomplete)?;
        let view = dto::View::try_from(view.store_view()).map_err(|_| Error::Corrupt)?;
        context.budget.metadata(&view)?;
        let pin = Arc::get_mut(&mut context.access).ok_or(Error::Unavailable)?;
        pin.inventory.insert(ObjectKey::Operation(parent));
        pin.inventory.insert(ObjectKey::View(data.view_id));
        // The delta follows only exact view heads and their guarded parent edges.
        pin.inventory
            .extend(view.head_ids.iter().copied().map(ObjectKey::Commit));
        pin.views.insert(data.view_id, view);
        pin.operations.insert(parent, data);
        let source = context
            .scoped
            .as_ref()
            .map_or(&context.store, |guard| &guard.source)
            .clone();
        let guard = scoped::Guard::new(
            source,
            &context.access,
            context.budget.clone(),
            context.limits,
        );
        context.store = guard.store();
        context.scoped = Some(guard);
    }
    context.dispatch(selected, request, false).await
}
