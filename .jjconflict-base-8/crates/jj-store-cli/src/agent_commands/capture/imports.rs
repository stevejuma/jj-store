use super::*;
#[derive(Clone, Debug, Args)]
pub struct ImportArgs {
    #[command(flatten)]
    pub intent: ControlArgs,
    #[arg(long, value_parser = sources::selector)]
    pub source: String,
}
#[derive(Clone, Debug, Args)]
pub struct StatusArgs {
    #[arg(long)]
    pub request_id: RequestId,
}
pub(super) async fn run(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    client: &client::Client,
    args: &CaptureCommand,
) -> Result<(), CommandError> {
    let (value, receipt, progress) = match args {
        CaptureCommand::Import(args) => {
            nonzero(args.intent.request_id)?;
            let request = CaptureImportCommand {
                version: 1,
                request_id: args.intent.request_id,
                generation: args.intent.policy.generation.clone(),
                expected_revision: args.intent.policy.expected_revision,
                selector: args.source.clone(),
            };
            let result: CaptureImportOutcome = client
                .request(
                    "POST",
                    "/v1/local/capture/import",
                    Some(serde_json::to_value(&request).map_err(error)?),
                )
                .await?;
            if result.version != 1
                || result.receipt.command != request
                || !valid(&result.receipt, &result.progress)
            {
                return Err(client::failure("capture import outcome unknown", true));
            }
            (
                serde_json::to_value(&result),
                Some(result.receipt),
                result.progress,
            )
        }
        CaptureCommand::ImportStatus(args) => {
            nonzero(args.request_id)?;
            let path = format!("/v1/local/capture/imports?request_id={}", args.request_id);
            let result: CaptureImportStatus = client.request("GET", &path, None).await?;
            if result.version != 1
                || result.request_id != args.request_id
                || result.receipt.as_ref().is_some_and(|r| {
                    r.command.request_id != args.request_id || !valid(r, &result.progress)
                })
                || (result.receipt.is_none()
                    && (result.progress.phase == CaptureImportPhase::Completed
                        || result.progress.boundary_available
                        || result.progress.committed_byte_offset != 0))
            {
                return Err(client::failure(
                    "capture import progress unavailable",
                    false,
                ));
            }
            (
                serde_json::to_value(&result),
                result.receipt,
                result.progress,
            )
        }
        _ => unreachable!(),
    };
    let mut text = format!(
        "Import {:?}; continuing tail {:?}.\n",
        progress.phase, progress.tail_phase
    );
    if let Some(receipt) = receipt {
        text += &format!(
            "Accepted request {}; receipt {}: {}/{} bytes published.\n",
            receipt.command.request_id,
            receipt.ordinal,
            progress.committed_byte_offset,
            receipt.complete_byte_offset
        );
    } else {
        text += "No durable acceptance observed.\n";
    }
    presentation::result(ui, helper, format, &value.map_err(error)?, &text)
}
fn nonzero(request: RequestId) -> Result<(), CommandError> {
    if request.as_bytes() == &[0; 16] {
        return Err(client::failure("request identity must be nonzero", true));
    }
    Ok(())
}
fn valid(receipt: &CaptureImportAcceptance, progress: &CaptureImportProgress) -> bool {
    receipt.command.version == 1
        && receipt.ordinal > 0
        && receipt.command.request_id.as_bytes() != &[0; 16]
        && generation(&receipt.command.generation).is_ok()
        && sources::selector(&receipt.command.selector).is_ok()
        && receipt.product_version.len() <= 32
        && !receipt.product_version.is_empty()
        && receipt.complete_records > 0
        && receipt.complete_byte_offset >= receipt.complete_records
        && progress.committed_byte_offset <= progress.captured_byte_offset
        && progress.captured_byte_offset <= receipt.complete_byte_offset
        && (progress.phase != CaptureImportPhase::Completed
            || (progress.boundary_available
                && progress.committed_byte_offset == receipt.complete_byte_offset))
        && progress.tail.as_ref().is_none_or(|t| {
            t.committed_byte_offset <= t.captured_byte_offset && t.outstanding_ranges <= 4096
        })
}
