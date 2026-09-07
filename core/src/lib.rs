use tracing_subscriber::{EnvFilter, FmtSubscriber};
pub mod i3_tools;
pub mod layouts;
pub mod window_tools;

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
