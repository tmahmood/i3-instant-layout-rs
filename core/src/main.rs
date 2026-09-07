use i3_instant_layout_rs::i3_tools::apply_layout;
use i3_instant_layout_rs::window_tools::{get_active_window, get_window_ids};
use i3_instant_layout_rs::set_tracing_full;

fn main() -> anyhow::Result<()> {
    set_tracing_full();
    let args = std::env::args().collect::<Vec<String>>();
    let layout_cmd = args.get(1).unwrap();
    let dry_run = args.get(2).is_some();
    let n = apply_layout(layout_cmd, dry_run)?;
    println!("{}", n);
    Ok(())
}
