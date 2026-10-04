use super::*;

#[derive(Clone, Debug, Args)]
pub struct SourceArgs {
    /// Page size (default 50, maximum 100)
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..=100))]
    pub limit: Option<u16>,
    /// Runtime-scoped continuation returned by the preceding page
    #[arg(long, value_parser = selector)]
    pub cursor: Option<String>,
}
pub(super) fn selector(value: &str) -> Result<String, String> {
    if value.len() != 49
        || value.as_bytes()[32] != b':'
        || value
            .bytes()
            .enumerate()
            .any(|(i, b)| i != 32 && !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
        || generation(&value[..32]).is_err()
        || value[33..] == "0".repeat(16)
    {
        return Err(
            "source cursor must be a runtime generation and nonzero lowercase hex ordinal".into(),
        );
    }
    Ok(value.into())
}
pub(super) async fn run(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    client: &client::Client,
    args: &SourceArgs,
) -> Result<(), CommandError> {
    let mut query = url::form_urlencoded::Serializer::new(String::new());
    if let Some(limit) = args.limit {
        query.append_pair("limit", &limit.to_string());
    }
    if let Some(cursor) = &args.cursor {
        query.append_pair("cursor", cursor);
    }
    let query = query.finish();
    let path = if query.is_empty() {
        "/v1/local/capture/sources".into()
    } else {
        format!("/v1/local/capture/sources?{query}")
    };
    let page: CaptureSourcePage = client.request("GET", &path, None).await?;
    validate(&page, args).map_err(|_| client::failure("capture source page unavailable", false))?;
    let value = serde_json::to_value(&page).map_err(error)?;
    let mut text = format!(
        "Capture {}: {} cached sources.\n",
        if page.enabled { "enabled" } else { "disabled" },
        page.total
    );
    for row in &page.sources {
        text += &format!(
            "{}: {} {:?}; reader {:?}; awaiting activity {}; caught up {}; degraded {}.\n",
            row.selector,
            profile_name(row.profile),
            row.product_version,
            row.phase,
            row.awaiting_activity,
            row.caught_up,
            row.degraded
        );
    }
    if let Some(cursor) = &page.next_cursor {
        text += &format!("Next cursor: {cursor}\n");
    }
    presentation::result(ui, helper, format, &value, &text)
}
fn validate(page: &CaptureSourcePage, args: &SourceArgs) -> Result<(), ()> {
    if page.version != 1
        || page.total > 4096
        || page.sources.len() > usize::from(args.limit.unwrap_or(50))
        || page.sources.len() > page.total
    {
        return Err(());
    }
    if !page.enabled {
        return if page.runtime_generation.is_none()
            && page.phase.is_none()
            && page.total == 0
            && page.sources.is_empty()
            && page.next_cursor.is_none()
        {
            Ok(())
        } else {
            Err(())
        };
    }
    let runtime = page.runtime_generation.as_deref().ok_or(())?;
    generation(runtime).map_err(|_| ())?;
    if page.phase.is_none() || args.cursor.as_ref().is_some_and(|c| &c[..32] != runtime) {
        return Err(());
    }
    let mut after = args.cursor.as_deref().unwrap_or("");
    for row in &page.sources {
        selector(&row.selector).map_err(|_| ())?;
        if &row.selector[..32] != runtime
            || row.selector.as_str() <= after
            || row.product_version.is_empty()
            || row.product_version.len() > 32
        {
            return Err(());
        }
        after = &row.selector;
    }
    if let Some(cursor) = &page.next_cursor {
        selector(cursor).map_err(|_| ())?;
        if page.sources.is_empty() || cursor != after || page.sources.len() == page.total {
            return Err(());
        }
    }
    Ok(())
}
