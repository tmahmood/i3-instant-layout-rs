use std::io::Write;
use std::process::Command;
use tempfile::tempfile;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

const LOG_FULL: &str = "i3_instant_layout_rs=trace";
const LOG_TEST: &str = "i3_instant_layout_rs=debug";

pub fn set_tracing_test() {
    let Ok(directive) = LOG_TEST.parse() else {
        eprintln!("failed to set global logger");
        return;
    };
    let env = EnvFilter::from_default_env().add_directive(directive);
    set_tracing(Some(env))
}

pub fn set_tracing_full() {
    let Ok(directive) = LOG_FULL.parse() else {
        eprintln!("failed to set global logger");
        return;
    };
    let env = EnvFilter::from_default_env().add_directive(directive);
    set_tracing(Some(env))
}
pub fn set_tracing(custom_env: Option<EnvFilter>) {
    let subscriber = FmtSubscriber::builder()
        .compact()
        .with_line_number(true)
        .with_env_filter(custom_env.unwrap_or_else(EnvFilter::from_default_env))
        .finish();
    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("failed to set global logger, {e}");
    }
}

pub fn append_layout() -> anyhow::Result<()> {
    let mut tmp_file = tempfile()?;
    tmp_file.write_all(b"Hello, world!")?;

    Ok(())
}

pub fn get_active_window() -> anyhow::Result<i32> {
    let output = String::from_utf8(
        Command::new("xdotool")
            .arg("getactivewindow")
            .output()?
            .stdout,
    )?;
    let output = output.trim().parse::<i32>()?;
    tracing::trace!("got: {output:?}");
    Ok(output)
}

pub fn focus_window(window_id: i32) -> anyhow::Result<()> {
    let id_txt = format!("[id='{}']", window_id);
    Command::new("i3-msg").args([&id_txt, "focus"]).output()?;
    Ok(())
}

pub fn get_window_ids() -> anyhow::Result<Vec<i32>> {
    let mut desktop = Command::new("xprop");
    let output = String::from_utf8(
        desktop
            .arg("-notype")
            .arg("-root")
            .arg("_NET_CURRENT_DESKTOP")
            .output()?
            .stdout,
    )?;
    let n = output
        .split('=')
        .skip(1)
        .next()
        .ok_or_else(|| anyhow::anyhow!("failed to parse output"))?
        .trim();
    let output = String::from_utf8(
        Command::new("xdotool")
            .args([
                "search",
                "--all",
                "--onlyvisible",
                "--desktop",
                n,
                "--class",
                "^.*",
            ])
            .output()?
            .stdout,
    )?;
    let ids = output
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    Ok(ids)
}

pub fn apply_layout(layout: &str) -> anyhow::Result<()> {
    let active = get_active_window()?;
    let mut windows = get_window_ids()?;
    tracing::trace!("> {windows:?}");
    windows.retain(|ii| ii != &active);
    let total_windows = windows.len();
    tracing::trace!("> {windows:?}");
    Ok(())
}
