
const CDEFAULT: &str = "\x1b[0m";
const CGRAY: &str = "\x1b[1;30m";


fn main() {
    // let date = chrono::Utc::now();
    // println!("{CGRAY}{date}{CDEFAULT}");
    // println!("{date:?}");
    // println!("{date:#?}");

    tracing::subscriber::set_global_default(App);

    tracing::info!("Oof");
    tracing::error!("awdoakwdo");
    tracing::debug!(name:"lag", "awdoakwdo");
}



struct App;

use tracing::span;

impl tracing::Subscriber for App {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        metadata.module_path().unwrap_or_default().starts_with("iter")
    }

    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        tracing::span::Id::from_u64(1)
    }

    fn record(&self, span: &span::Id, values: &span::Record<'_>) {
    }

    fn event(&self, event: &tracing::Event<'_>) {
        println!("Event: {event:#?}");
    }

    fn enter(&self, span: &span::Id) {
    }

    fn exit(&self, span: &span::Id) {
    }

    fn drop_span(&self, _id: span::Id) {
    }

    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
    }
}

