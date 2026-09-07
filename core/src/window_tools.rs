use std::io::Write;
use std::process::Command;



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
