//! Fixed application workspace command: pin the source, checkout only destination.
use jj_cli::{
    cli_util::CommandHelper,
    command_error::{CommandError, user_error},
};
use jj_lib::{
    backend::CommitId, op_store::OperationId, ref_name::WorkspaceNameBuf, repo::Repo as _,
    workspace::Workspace,
};
use std::path::PathBuf;
#[derive(Clone, Debug, clap::Args)]
pub struct Args {
    /// Exact full source operation ID; does not snapshot source working files.
    #[arg(long)]
    pub operation: String,
    /// Exact full starting revision ID for the isolated checkout.
    #[arg(long)]
    pub revision: String,
    /// Application-owned agent workspace name.
    #[arg(long)]
    pub name: String,
    /// New absolute physical workspace destination.
    pub destination: PathBuf,
}
pub async fn run(helper: &CommandHelper, args: Args) -> Result<(), CommandError> {
    if !args.name.starts_with("agent-")
        || args.name.len() > 80
        || !args
            .name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        || !args.destination.is_absolute()
        || !jj_store_service::agent_execution::valid_commit(&args.revision)
        || args.operation.len() != 128
        || !args.operation.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err(user_error("Invalid isolated agent workspace selection"));
    }
    if args.destination.exists() {
        return Err(user_error("Agent workspace destination already exists"));
    }
    // load_workspace() loads metadata; workspace_helper() would snapshot the source.
    let source = helper.load_workspace()?;
    let loader = source.repo_loader();
    let operation = loader
        .load_operation(&OperationId::from_bytes(
            &hex::decode(&args.operation).map_err(|_| user_error("Invalid agent operation"))?,
        ))
        .await?;
    let repo = loader.load_at(&operation).await?;
    let selected = repo
        .store()
        .get_commit_async(&CommitId::from_bytes(
            &hex::decode(&args.revision).map_err(|_| user_error("Invalid agent revision"))?,
        ))
        .await?;
    let name: WorkspaceNameBuf = args.name.into();
    if repo.view().get_wc_commit_id(&name).is_some() {
        return Err(user_error("Agent workspace name already exists"));
    }
    std::fs::create_dir(&args.destination)
        .map_err(|_| user_error("Agent workspace destination unavailable"))?;
    let (mut destination, repo) = Workspace::init_workspace_with_existing_repo(
        &args.destination,
        source.repo_path(),
        &repo,
        helper.get_working_copy_factory()?,
        name.clone(),
    )
    .await?;
    let mut tx = repo.start_transaction();
    let commit = tx.repo_mut().check_out(name, &selected).await?;
    tx.repo_mut().rebase_descendants().await?;
    let published = tx.commit("create isolated agent workspace").await?;
    destination
        .check_out(published.op_id().clone(), None, &commit)
        .await
        .map_err(|_| user_error("Agent destination checkout outcome unknown"))?;
    Ok(())
}
