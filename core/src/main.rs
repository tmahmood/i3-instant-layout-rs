use i3_instant_layout_rs::{append_layout, apply_layout, get_active_window, get_window_ids, set_tracing_full};

fn main() -> anyhow::Result<()> {
    set_tracing_full();
    get_window_ids()?;
    get_active_window()?;
    apply_layout("some")?;
    Ok(())
}
