//! jj-nostr presentation conventions around upstream CliRunner. Only the custom
//! command family is intercepted; ordinary command parsing remains upstream.
use crate::commands::{Extension, Format};
use clap::Subcommand as _;
use jj_cli::{
    cli_util::{BoxedAsyncCliDispatch, CommandHelper},
    command_error::{CommandError, user_error},
    ui::Ui,
};
use jj_lib::config::{ConfigLayer, ConfigSource, StackedConfig};
use std::io::Write as _;

/// Native text output owns a short-lived pager process so a failed spawn can
/// fall back to stdout without losing the result. JSON never enters this path.
fn page(ui: &Ui, text: &str, no_pager: bool) -> Result<(), CommandError> {
    use std::{
        io::IsTerminal as _,
        process::{Command, Stdio},
    };
    let pager = std::env::var("JJ_STORE_PAGER").ok();
    if no_pager
        || !std::io::stdout().is_terminal()
        || pager
            .as_deref()
            .is_some_and(|value| value.is_empty() || value == "cat")
    {
        write!(ui.stdout(), "{text}")?;
        return Ok(());
    }
    let words = pager
        .as_deref()
        .map(shlex::split)
        .unwrap_or_else(|| Some(vec!["less".into(), "-FRX".into()]));
    let Some(words) = words.filter(|words| !words.is_empty()) else {
        writeln!(
            ui.warning_default(),
            "Invalid JJ_STORE_PAGER command; writing to stdout."
        )?;
        writeln!(
            ui.hint_default(),
            "Pass --no-pager or set JJ_STORE_PAGER=cat."
        )?;
        write!(ui.stdout(), "{text}")?;
        return Ok(());
    };
    match Command::new(&words[0])
        .args(&words[1..])
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            let written = child
                .stdin
                .take()
                .expect("piped pager stdin")
                .write_all(text.as_bytes());
            let waited = child.wait();
            if let Err(error) = written
                && error.kind() != std::io::ErrorKind::BrokenPipe
            {
                return Err(error.into());
            }
            waited?;
        }
        Err(error) => {
            writeln!(
                ui.warning_default(),
                "Could not start native pager: {error}"
            )?;
            writeln!(
                ui.hint_default(),
                "Pass --no-pager or set JJ_STORE_PAGER=cat."
            )?;
            write!(ui.stdout(), "{text}")?;
        }
    }
    Ok(())
}
pub fn text(ui: &Ui, helper: &CommandHelper, value: &str) -> Result<(), CommandError> {
    let no_pager = helper.global_args().early_args.no_pager == Some(true)
        || helper
            .settings()
            .config()
            .get::<String>("ui.paginate")
            .is_ok_and(|value| value == "never");
    page(ui, value, no_pager)
}
pub fn result(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    value: &serde_json::Value,
    message: &str,
) -> Result<(), CommandError> {
    if format == Format::Json {
        writeln!(
            ui.stdout(),
            "{}",
            serde_json::to_string(value).expect("JSON value")
        )?;
        Ok(())
    } else {
        text(ui, helper, message)
    }
}
pub fn help(ui: &mut Ui, helper: &CommandHelper, path: &[String]) -> Result<(), CommandError> {
    let mut app = helper.app().clone();
    app.build();
    let mut command = app
        .find_subcommand_mut("store")
        .expect("registered store command");
    for name in path {
        command = command.find_subcommand_mut(name).ok_or_else(|| {
            user_error("Unknown native help command").hinted("Use 'jj store help -k commands'.")
        })?;
    }
    let rendered = command.render_long_help();
    let raw = if ui.color() {
        rendered.ansi().to_string()
    } else {
        rendered.to_string()
    };
    text(
        ui,
        helper,
        &format!(
            "{}\nUse 'jj store help -k commands' for the native command guide.\n",
            markdown(&raw, ui.color())
        ),
    )
}
pub async fn dispatch(
    ui: &mut Ui,
    helper: &CommandHelper,
    inner: BoxedAsyncCliDispatch<'_>,
) -> Result<(), CommandError> {
    if let Some(matches) = helper.matches().subcommand_matches("help") {
        let path: Vec<_> = matches
            .get_many::<String>("command")
            .into_iter()
            .flatten()
            .cloned()
            .collect();
        if path.first().is_some_and(|name| name == "store") {
            return help(ui, helper, &path[1..]);
        }
    }
    inner.call(ui, helper).await
}
/// Clap emits help before dispatch hooks. Preflight renders only explicit
/// custom-command help/errors. Alias expansion and ordinary commands remain
/// entirely with CliRunner, and valid store commands still dispatch there.
pub fn preflight() -> Option<u8> {
    let args: Vec<_> = std::env::args_os().collect();
    let app = Extension::augment_subcommands(jj_cli::commands::default_app());
    let probe: Vec<_> = args
        .iter()
        .filter(|arg| *arg != "--help" && *arg != "-h")
        .cloned()
        .collect();
    let Ok(matches) = app.clone().ignore_errors(true).try_get_matches_from(probe) else {
        return None;
    };
    if matches.subcommand_name() != Some("store") {
        return None;
    }
    let error = match app.clone().try_get_matches_from(&args) {
        Ok(_) => return None,
        Err(error) => error,
    };
    if error.kind() != clap::error::ErrorKind::DisplayHelp {
        let _ = error.print();
        eprintln!("Hint: Use 'jj store help <command>' or 'jj store help -k commands'.");
        return Some(2);
    }
    let mut config = StackedConfig::empty();
    for layer in jj_cli::config::default_config_layers() {
        config.add_layer(layer);
    }
    let no_pager = matches
        .get_one::<bool>("no_pager")
        .copied()
        .unwrap_or(false);
    let mut color = "auto";
    for (i, arg) in args.iter().enumerate() {
        if arg == "--color" {
            color = args.get(i + 1).and_then(|a| a.to_str()).unwrap_or("auto");
        }
        if let Some(value) = arg.to_str().and_then(|a| a.strip_prefix("--color=")) {
            color = value;
        }
    }
    if color == "auto" && std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()) {
        color = "never";
    }
    config.add_layer(
        ConfigLayer::parse(ConfigSource::CommandArg, &format!("ui.color = '{color}'")).ok()?,
    );
    let ui = Ui::with_config(&config).ok()?;
    let rendered = error.render();
    let raw = if ui.color() {
        rendered.ansi().to_string()
    } else {
        rendered.to_string()
    };
    let mut output = markdown(&raw, ui.color());
    if args.iter().any(|arg| arg == "--help") {
        output.push_str("\nUse 'jj store help -k commands' for the native command guide.\n");
    }
    Some(if page(&ui, &output, no_pager).is_ok() {
        0
    } else {
        1
    })
}

/// A small renderer for our authored command documents. Literal code lines keep
/// their indentation; styles are applied only under the active terminal policy.
pub fn markdown(text: &str, color: bool) -> String {
    let mut out = String::new();
    let mut fence = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            fence = !fence;
            continue;
        }
        if fence {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let trimmed = line.trim_start();
        let hashes = trimmed.chars().take_while(|c| *c == '#').count();
        let heading = (1..=6).contains(&hashes) && trimmed.as_bytes().get(hashes) == Some(&b' ');
        let line = if heading {
            trimmed[hashes + 1..].to_owned()
        } else {
            line.to_owned()
        };
        let mut inline = String::new();
        let mut rest = line.as_str();
        while !rest.is_empty() {
            let delimiter = if rest.starts_with("**") {
                Some("**")
            } else if rest.starts_with('`') {
                Some("`")
            } else {
                None
            };
            if let Some(mark) = delimiter
                && let Some(end) = rest[mark.len()..].find(mark)
            {
                let content = &rest[mark.len()..mark.len() + end];
                if color {
                    inline.push_str("\x1b[1m");
                }
                inline.push_str(content);
                if color {
                    inline.push_str("\x1b[0m");
                }
                rest = &rest[end + 2 * mark.len()..];
                continue;
            }
            // Clap may wrap a code span across lines. Strip the split marker
            // even when its matching delimiter is on the neighboring line.
            if let Some(mark) = delimiter {
                rest = &rest[mark.len()..];
                continue;
            }
            if rest.starts_with('[')
                && let Some(end) = rest.find("](")
                && let Some(close) = rest[end + 2..].find(')')
            {
                inline.push_str(&rest[1..end]);
                inline.push_str(" (");
                inline.push_str(&rest[end + 2..end + 2 + close]);
                inline.push(')');
                rest = &rest[end + 3 + close..];
                continue;
            }
            let next = rest.chars().next().unwrap();
            inline.push(next);
            rest = &rest[next.len_utf8()..];
        }
        if heading && color {
            out.push_str("\x1b[1;33m");
        }
        out.push_str(&inline);
        if heading && color {
            out.push_str("\x1b[0m");
        }
        out.push('\n');
    }
    out
}
