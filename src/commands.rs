use std::fs;
use std::path::PathBuf;

use serde_json::{Map, Value};
use zed_extension_api::{
    self as zed, SlashCommandArgumentCompletion, SlashCommandOutput, SlashCommandOutputSection,
    Worktree,
};

use crate::config::ResolvedSettings;
use crate::utils::path_to_string;

pub const DCM_SLASH_COMMAND: &str = "dcm";

const OPEN_RULES_URL: &str = "https://dcm.dev/docs/rules/";
const OPEN_METRICS_URL: &str = "https://dcm.dev/docs/metrics/";
const FEEDBACK_URL: &str = "https://discord.gg/Vzjprgk4sb";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ToggleTarget {
    Baseline,
    UnusedCode,
    UnusedFiles,
    NewVersion,
}

pub fn complete(args: &[String]) -> Vec<SlashCommandArgumentCompletion> {
    match args.len() {
        0 => top_level_completions(""),
        1 => top_level_completions(&args[0]),
        _ => match args[0].as_str() {
            "open" => subcommand_completions(&["rules", "metrics", "feedback", "logs"], args),
            "toggle" => subcommand_completions(
                &["baseline", "unused-code", "unused-files", "new-version"],
                args,
            ),
            "log" => subcommand_completions(&["capture", "clear"], args),
            _ => Vec::new(),
        },
    }
}

fn top_level_completions(input: &str) -> Vec<SlashCommandArgumentCompletion> {
    let options = [
        ("help", "help", false),
        ("open", "open ", false),
        ("toggle", "toggle ", false),
        ("restart", "restart", true),
        ("log", "log ", false),
    ];

    options
        .iter()
        .filter(|(label, _, _)| label.starts_with(input))
        .map(|(label, new_text, run)| SlashCommandArgumentCompletion {
            label: label.to_string(),
            new_text: new_text.to_string(),
            run_command: *run,
        })
        .collect()
}

fn subcommand_completions(
    options: &[&str],
    args: &[String],
) -> Vec<SlashCommandArgumentCompletion> {
    let last = args.last().map(String::as_str).unwrap_or("");

    options
        .iter()
        .filter(|item| item.starts_with(last))
        .map(|item| SlashCommandArgumentCompletion {
            label: (*item).to_string(),
            new_text: format!("{} {}", args[..args.len() - 1].join(" "), item)
                .trim()
                .to_string(),
            run_command: args.len() > 1,
        })
        .collect()
}

pub fn run(args: Vec<String>, worktree: Option<&Worktree>) -> zed::Result<SlashCommandOutput> {
    let Some(worktree) = worktree else {
        return Err("DCM commands require an active worktree".to_string());
    };

    if args.is_empty() {
        return Ok(help_output());
    }

    match args[0].as_str() {
        "help" => Ok(help_output()),
        "open" => run_open(&args[1..]),
        "toggle" => run_toggle(&args[1..], worktree),
        "log" => run_log(&args[1..], worktree),
        "restart" => Ok(info_output(
            "Restart DCM",
            "Use `Zed: Restart Language Server` from the command palette to restart the DCM server.",
        )),
        other => Err(format!("Unknown DCM subcommand `{other}`. Run `dcm help` for options.")),
    }
}

fn run_open(args: &[String]) -> zed::Result<SlashCommandOutput> {
    if args.is_empty() {
        return Err("Specify what to open: rules, metrics, feedback, or logs".to_string());
    }

    let (title, message) = match args[0].as_str() {
        "rules" => ("DCM Rules", format!("Documentation: {OPEN_RULES_URL}")),
        "metrics" => ("DCM Metrics", format!("Documentation: {OPEN_METRICS_URL}")),
        "feedback" => (
            "DCM Feedback",
            format!("Join the community: {FEEDBACK_URL}"),
        ),
        "logs" => (
            "DCM Logs",
            "Log file is located under `.zed/dcm.log` when capture is enabled.".to_string(),
        ),
        other => {
            return Err(format!(
                "Unknown open target `{other}`. Use rules, metrics, feedback, or logs."
            ))
        }
    };

    Ok(info_output(title, &message))
}

fn run_toggle(args: &[String], worktree: &Worktree) -> zed::Result<SlashCommandOutput> {
    if args.is_empty() {
        return Err(
            "Specify toggle target: baseline, unused-code, unused-files, or new-version"
                .to_string(),
        );
    }

    let target = match args[0].as_str() {
        "baseline" => ToggleTarget::Baseline,
        "unused-code" => ToggleTarget::UnusedCode,
        "unused-files" => ToggleTarget::UnusedFiles,
        "new-version" => ToggleTarget::NewVersion,
        other => {
            return Err(format!(
                "Unknown toggle target `{other}`. Use baseline, unused-code, unused-files, or new-version."
            ))
        }
    };

    let settings = ResolvedSettings::from_worktree(worktree)?;
    let mut root_obj = read_settings_map(worktree)?;
    let mut dcm_map = root_obj
        .remove("dcm")
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_else(Map::new);

    let (title, message) = match target {
        ToggleTarget::Baseline => {
            let new_value = !settings.user.disable_baseline;
            dcm_map.insert("disable_baseline".to_string(), Value::Bool(new_value));
            (
                "Baseline Toggle",
                if new_value {
                    "Baseline filtering disabled. Restart the DCM server for changes to take effect."
                } else {
                    "Baseline filtering enabled. Restart the DCM server for changes to take effect."
                },
            )
        }
        ToggleTarget::UnusedCode => {
            let new_value = !settings.user.show_unused_code;
            dcm_map.insert("show_unused_code".to_string(), Value::Bool(new_value));
            (
                "Unused Code Toggle",
                if new_value {
                    "Unused code issues will be reported after restarting the DCM server."
                } else {
                    "Unused code issues suppressed. Restart the DCM server for changes to take effect."
                },
            )
        }
        ToggleTarget::UnusedFiles => {
            let new_value = !settings.user.show_unused_files;
            dcm_map.insert("show_unused_files".to_string(), Value::Bool(new_value));
            (
                "Unused Files Toggle",
                if new_value {
                    "Unused file analysis enabled. Restart the DCM server for changes to take effect."
                } else {
                    "Unused file analysis disabled. Restart the DCM server for changes to take effect."
                },
            )
        }
        ToggleTarget::NewVersion => {
            let new_value = !settings.user.show_new_version;
            dcm_map.insert("show_new_version".to_string(), Value::Bool(new_value));
            (
                "Version Notification Toggle",
                if new_value {
                    "New version notifications enabled."
                } else {
                    "New version notifications disabled."
                },
            )
        }
    };

    write_settings_map(worktree, root_obj, dcm_map)?;
    Ok(info_output(title, message))
}

fn run_log(args: &[String], worktree: &Worktree) -> zed::Result<SlashCommandOutput> {
    if args.is_empty() {
        return Err("Specify log command: capture or clear".to_string());
    }

    let mut root_obj = read_settings_map(worktree)?;
    let mut dcm_map = root_obj
        .remove("dcm")
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_else(Map::new);

    match args[0].as_str() {
        "capture" => {
            let log_path = default_log_path(worktree);
            dcm_map.insert(
                "log_file_path".to_string(),
                Value::String(path_to_string(&log_path)?),
            );
            write_settings_map(worktree, root_obj, dcm_map)?;
            Ok(info_output(
                "Log Capture Enabled",
                &format!(
                    "Server communication will be captured to {}. Restart DCM to begin logging.",
                    path_to_string(&log_path)?
                ),
            ))
        }
        "clear" => {
            dcm_map.remove("log_file_path");
            write_settings_map(worktree, root_obj, dcm_map)?;
            Ok(info_output(
                "Log Capture Disabled",
                "DCM log capture disabled. Delete existing log files manually if desired.",
            ))
        }
        other => Err(format!(
            "Unknown log command `{other}`. Use capture or clear."
        )),
    }
}

fn help_output() -> SlashCommandOutput {
    let text = r#"DCM commands:
- dcm help
- dcm open [rules|metrics|feedback|logs]
- dcm toggle [baseline|unused-code|unused-files|new-version]
- dcm log [capture|clear]
- dcm restart"#;

    SlashCommandOutput {
        text: text.to_string(),
        sections: Vec::new(),
    }
}

fn info_output(title: &str, message: &str) -> SlashCommandOutput {
    SlashCommandOutput {
        text: format!("{title}: {message}"),
        sections: vec![SlashCommandOutputSection {
            range: zed::Range {
                start: 0,
                end: title.len() as u32,
            },
            label: title.to_string(),
        }],
    }
}

fn read_settings_map(worktree: &Worktree) -> zed::Result<Map<String, Value>> {
    let path = settings_file_path(worktree);
    if path.exists() {
        let contents = fs::read_to_string(&path)
            .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;
        if contents.trim().is_empty() {
            Ok(Map::<String, Value>::new())
        } else {
            let value: Value = serde_json::from_str(&contents)
                .map_err(|err| format!("Invalid JSON in {}: {err}", path.display()))?;
            if let Some(map) = value.as_object() {
                Ok(map.clone())
            } else {
                Err(format!(
                    "Settings file {} must contain a JSON object",
                    path.display()
                ))
            }
        }
    } else {
        Ok(Map::<String, Value>::new())
    }
}

fn write_settings_map(
    worktree: &Worktree,
    mut root_obj: Map<String, Value>,
    dcm_map: Map<String, Value>,
) -> zed::Result<()> {
    let path = settings_file_path(worktree);
    root_obj.insert("dcm".to_string(), Value::Object(dcm_map));
    let final_root = Value::Object(root_obj);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            format!(
                "Failed to create settings directory {}: {err}",
                parent.display()
            )
        })?;
    }

    let serialized = serde_json::to_string_pretty(&final_root)
        .map_err(|err| format!("Failed to serialize settings: {err}"))?;
    fs::write(&path, serialized)
        .map_err(|err| format!("Failed to write settings file {}: {err}", path.display()))
}

fn settings_file_path(worktree: &Worktree) -> PathBuf {
    let root = PathBuf::from(worktree.root_path());
    root.join(".zed").join("settings.json")
}

fn default_log_path(worktree: &Worktree) -> PathBuf {
    PathBuf::from(worktree.root_path())
        .join(".zed")
        .join("dcm.log")
}
