//! Human-readable native operation metadata; command payloads are not log text.
use super::{Command, Placement, config::Config};

fn label(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn placement(value: Placement) -> &'static str {
    match value {
        Placement::Above => "above",
        Placement::Below => "below",
    }
}

pub(super) fn describe(command: &Command, config: &Config) -> String {
    let workspace_name = |id: &str| {
        label(
            config
                .workspaces
                .iter()
                .find(|w| w.id == id)
                .map_or(id, |w| &w.name),
        )
    };
    let action = match command {
        Command::Refresh => "Refresh workspace".into(),
        Command::Reconcile => "Reconcile workspace branches".into(),
        // Native history::restore supplies its own machine-readable descriptions.
        Command::Undo => "Undo operation".into(),
        Command::Redo => "Redo operation".into(),
        Command::Setup { name, target } => format!(
            "Set up workspace {} targeting {}",
            label(name),
            label(target)
        ),
        Command::Create { name } => format!("Create workspace {}", label(name)),
        Command::Switch { workspace } => {
            format!("Switch to workspace {}", workspace_name(workspace))
        }
        Command::Delete { workspace } => format!("Delete workspace {}", workspace_name(workspace)),
        Command::Update { name, target, .. } => format!(
            "Update workspace {} targeting {}",
            label(name),
            label(target)
        ),
        Command::Apply { branch } => format!("Apply {}", label(branch)),
        Command::Unapply { branch } => format!("Unapply {}", label(branch)),
        Command::UnapplyBranches { branches } => format!(
            "Unapply {}",
            branches
                .iter()
                .map(|b| label(b))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Command::SetTarget { branch } => format!("Set target branch to {}", label(branch)),
        Command::Commit { branch, .. } => format!("Create commit on {}", label(branch)),
        Command::CreateStack {
            name,
            parent: Some(parent),
        } => format!("Create stack {} above {}", label(name), label(parent)),
        Command::CreateStack { name, parent: None } => format!("Create stack {}", label(name)),
        Command::CreateBranch {
            name,
            commit,
            placement: position,
        } => format!(
            "Create branch {} {} commit {}",
            label(name),
            placement(*position),
            label(commit)
        ),
        Command::RenameBranch { branch, name } => {
            format!("Rename branch {} to {}", label(branch), label(name))
        }
        Command::DeleteBranch { branch } => format!("Delete branch {}", label(branch)),
        Command::Rebase { branch } => {
            format!("Rebase branch {} onto workspace target", label(branch))
        }
        Command::SquashBranch { branch } => format!("Squash branch {}", label(branch)),
        Command::Reword { commit, .. } => format!("Reword commit {}", label(commit)),
        Command::Empty {
            commit,
            placement: position,
            ..
        } => format!(
            "Create empty commit {} commit {}",
            placement(*position),
            label(commit)
        ),
        Command::Move {
            commit,
            target,
            placement: position,
        } => format!(
            "Move commit {} {} commit {}",
            label(commit),
            placement(*position),
            label(target)
        ),
        Command::Squash { commit, target } => {
            format!("Squash commit {} into {}", label(commit), label(target))
        }
        Command::Uncommit { commit } => format!("Uncommit {}", label(commit)),
        Command::Abandon { commit } => format!("Abandon commit {}", label(commit)),
        Command::Transfer { target, .. } => {
            format!("Move selected changes into commit {}", label(target))
        }
        Command::Split { commit, .. } => format!("Split commit {}", label(commit)),
        Command::Edit { commit, .. } => format!("Edit commit {}", label(commit)),
        Command::Discard { .. } => "Discard selected changes".into(),
        Command::Resolve { path, .. } => format!("Resolve conflicts in {}", label(path)),
    };
    format!("Stacks: {action}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::local_stacks::Selection;

    #[test]
    fn descriptions_omit_content_messages_and_selection_tokens() {
        let selection = Selection {
            source: "source-id".into(),
            path: "file.txt".into(),
            hunks: Some(vec!["opaque-hunk-token".into()]),
        };
        let commands = [
            (
                Command::Resolve {
                    path: "file.txt".into(),
                    content: "private resolved contents\nsecond line".into(),
                },
                "Stacks: Resolve conflicts in file.txt",
            ),
            (
                Command::Commit {
                    branch: "feature/topic".into(),
                    message: "arbitrary message\nbody".into(),
                    selections: vec![selection.clone()],
                },
                "Stacks: Create commit on feature/topic",
            ),
            (
                Command::Edit {
                    commit: "abc123".into(),
                    message: "arbitrary message".into(),
                    selections: vec![selection.clone()],
                },
                "Stacks: Edit commit abc123",
            ),
            (
                Command::Transfer {
                    target: "abc123".into(),
                    selections: vec![selection.clone()],
                },
                "Stacks: Move selected changes into commit abc123",
            ),
            (
                Command::Discard {
                    selections: vec![selection],
                },
                "Stacks: Discard selected changes",
            ),
        ];
        for (command, expected) in commands {
            assert_eq!(describe(&command, &Config::default()), expected);
        }
    }

    #[test]
    fn descriptions_normalize_labels_and_spell_out_placement() {
        assert_eq!(
            describe(
                &Command::Setup {
                    name: "  My\n workspace\t ".into(),
                    target: "main".into()
                },
                &Config::default()
            ),
            "Stacks: Set up workspace My workspace targeting main"
        );
        for (position, word) in [(Placement::Above, "above"), (Placement::Below, "below")] {
            assert_eq!(
                describe(
                    &Command::Move {
                        commit: "abc123".into(),
                        target: "def456".into(),
                        placement: position
                    },
                    &Config::default()
                ),
                format!("Stacks: Move commit abc123 {word} commit def456")
            );
        }
    }
}
