//! Development-only documentation inventory, built from the registered parsers.
use clap::{Command, CommandFactory as _, Subcommand as _};
use serde_json::{Value, json};

fn command(command: &Command, path: &str) -> Value {
    json!({
        "path": path,
        "about": command.get_long_about().or(command.get_about()).map(ToString::to_string),
        "usage": command.clone().render_usage().to_string(),
        "aliases": command.get_visible_aliases().collect::<Vec<_>>(),
        "arguments": command.get_arguments().filter(|arg| !arg.is_hide_set()).map(|arg| json!({
            "id": arg.get_id().as_str(), "long": arg.get_long(), "short": arg.get_short(),
            "aliases": arg.get_visible_aliases(), "short_aliases": arg.get_visible_short_aliases(),
            "required": arg.is_required_set(), "global": arg.is_global_set(),
            "help": arg.get_long_help().or(arg.get_help()).map(ToString::to_string),
            "values": arg.get_value_names().map(|names| names.iter().map(ToString::to_string).collect::<Vec<_>>()),
            "defaults": arg.get_default_values().iter().map(|value| value.to_string_lossy()).collect::<Vec<_>>(),
            "possible": arg.get_value_parser().possible_values().map(|values| values.filter(|value| !value.is_hide_set()).map(|value| value.get_name().to_owned()).collect::<Vec<_>>()),
        })).collect::<Vec<_>>(),
        "commands": command.get_subcommands().filter(|sub| !sub.is_hide_set()).map(|sub| command_value(sub, path)).collect::<Vec<_>>()
    })
}

fn command_value(sub: &Command, parent: &str) -> Value {
    command(sub, &format!("{parent} {}", sub.get_name()))
}

fn main() {
    let mut client =
        jj_store_cli::commands::Extension::augment_subcommands(jj_cli::commands::default_app());
    client.build();
    let mut server = jj_store_cli::server_commands::ServerCli::command();
    server.build();
    let extensions = jj_store_cli::commands::Extension::augment_subcommands(Command::new("jj"));
    let families = extensions
        .get_subcommands()
        .map(|sub| {
            command_value(
                client
                    .find_subcommand(sub.get_name())
                    .expect("registered extension"),
                "jj",
            )
        })
        .collect::<Vec<_>>();
    println!("{}", serde_json::to_string_pretty(&json!({
        "version": 1, "client": families, "server": command(&server, "jj-store"),
        "configuration_defaults": {
            "limits": jj_store_server::config::Limits::default(),
            "replication": jj_store_server::config::ReplicationLimits::default(),
            "agent_capture": jj_store_server::capture_config::AgentCaptureConfig::default(),
            "search_example": serde_json::from_value::<jj_store_server::search::Repository>(json!({"repository": "00000000000000000000000000000001"})).unwrap(),
        }
    })).unwrap());
}
