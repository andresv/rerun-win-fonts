use re_viewer::external::{
    arrow2, eframe, re_entity_db, re_log, re_log_types, re_memory, re_types,
};

mod app;
mod rrlog;

// By using `re_memory::AccountingAllocator` Rerun can keep track of exactly how much memory it is using,
// and prune the data store when it goes above a certain limit.
// By using `mimalloc` we get faster allocations.
#[global_allocator]
static GLOBAL: re_memory::AccountingAllocator<mimalloc::MiMalloc> =
    re_memory::AccountingAllocator::new(mimalloc::MiMalloc);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = env_logger::Builder::from_default_env();
    if std::env::var("RUST_LOG").is_err() {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.format_timestamp_millis().init();

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_app_id("winrerun"),
        ..re_viewer::native::eframe_options(None)
    };

    // Install handlers for panics and crashes that prints to stderr and send
    // them to Rerun analytics (if the `analytics` feature is on in `Cargo.toml`).
    re_crash_handler::install_crash_handlers(re_viewer::build_info());

    // Listen for TCP connections from Rerun's logging SDKs.
    // There are other ways of "feeding" the viewer though - all you need is a `re_smart_channel::Receiver`.
    let rx = re_sdk_comms::serve(
        "0.0.0.0",
        re_sdk_comms::DEFAULT_SERVER_PORT,
        Default::default(),
    )?;

    let startup_options = re_viewer::StartupOptions::default();

    // This is used for analytics, if the `analytics` feature is on in `Cargo.toml`
    let app_env = re_viewer::AppEnvironment::Custom("My Wrapper".to_owned());

    eframe::run_native(
        "Winrerun",
        native_options,
        Box::new(move |cc| {
            re_viewer::customize_eframe_and_setup_renderer(cc)?;
            let mut rerun_app = re_viewer::App::new(
                re_viewer::build_info(),
                &app_env,
                startup_options,
                cc.egui_ctx.clone(),
                cc.storage,
            );
            rerun_app.add_receiver(rx);
            Ok(Box::new(app::TemplateApp::new(cc, rerun_app)))
        }),
    )?;

    Ok(())
}
