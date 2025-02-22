use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_tracing() {
    // Start configuring a fmt
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on with_thread_ids (true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
