use crate::layouts::parse_layout_command;
use crate::window_tools::{get_active_window, get_window_ids};
use std::io::Write;
use std::process::Command;

pub fn apply_layout(layout: &str, dry_run: bool) -> anyhow::Result<String> {
    let active = get_active_window()?;
    let mut windows = get_window_ids()?;
    tracing::trace!("> {windows:?}");
    if !windows.contains(&active) {
        windows.push(active);
    }
    let total_windows = windows.len();
    let layout = parse_layout_command(layout, total_windows as i32);
    tracing::trace!("> {windows:?} {total_windows}");
    let r = serde_json::to_string(&layout)?;
    if dry_run {
        return Ok(r);
    }
    let mut unmap_cmd = vec!["xdotool".to_string()];
    let mut map_cmd = vec!["xdotool".to_string()];

    append_layout(&r, total_windows as i32)?;
    for window_id in &windows {
        unmap_cmd.push("windowunmap".to_string());
        unmap_cmd.push(window_id.to_string());
        map_cmd.push("windowmap".to_string());
        map_cmd.push(window_id.to_string());
    }
    Command::new(&unmap_cmd[0]).args(&unmap_cmd[1..]).output()?;
    Command::new(&map_cmd[0]).args(&map_cmd[1..]).output()?;
    focus_window(active)?;
    Ok(r)
}
pub fn append_layout(layout: &str, window_count: i32) -> anyhow::Result<()> {
    let mut tmp_file = tempfile::NamedTempFile::new()?;
    tmp_file.write_all(layout.as_bytes())?;
    let c = Command::new("i3-msg")
        .arg("append_layout")
        .arg(tmp_file.path())
        .output()?;
    if !c.status.success() {
        anyhow::bail!("i3-msg failed: {}", String::from_utf8_lossy(&c.stderr));
    }
    Ok(())
}
pub fn focus_window(window_id: i32) -> anyhow::Result<()> {
    let id_txt = format!("[id='{}']", window_id);
    Command::new("i3-msg").args([&id_txt, "focus"]).output()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::i3_tools::apply_layout;
    use crate::set_tracing_full;

    const BUILT_STR: &str = r#"[{"layout": "splith", "type": "con", "nodes": [[{"layout": "splitv", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}]}], [{"layout": "splitv", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}]}], [{"layout": "splitv", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}]}]]}]"#;

    #[test]
    fn building_layout_json_file() -> anyhow::Result<()> {
        set_tracing_full();
        let ss = apply_layout("4k", false)?;
        let mut r: serde_json::Value = serde_json::from_str(&ss)?;
        r.sort_all_objects();
        let mut s: serde_json::Value = serde_json::from_str(BUILT_STR)?;
        s.sort_all_objects();
        assert_eq!(serde_json::to_string(&r)?, serde_json::to_string(&s)?);
        Ok(())
    }
}
