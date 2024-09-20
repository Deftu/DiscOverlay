use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    init_config, Config, Handle,
};
use tauri::{AppHandle, Manager};

const TRIGGER_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

pub fn setup() -> Handle {
    init_config(create_config(None)).unwrap()
}

pub fn setup_with_app(handle: Handle, app: &AppHandle) {
    handle.set_config(create_config(Some(app)));
}

fn create_encoder() -> PatternEncoder {
    PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}] [{h({l})}] {m}{n}")
}

fn create_file_appender(app: &Option<&AppHandle>) -> RollingFileAppender {
    let archive_pattern = match app {
        Some(app) => {
            let path = app.path();
            let log_dir = path.app_log_dir().expect("Failed to get app log dir");
            log_dir
                .join("output.{}.gz")
                .to_str()
                .expect("Failed to convert path to string")
                .to_string() // Convert to String
        }
        None => "log/output.{}.gz".to_string(),
    };

    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder()
        .build(archive_pattern.as_str(), 1)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let output_path = match app {
        Some(app) => {
            let path = app.path();
            let log_dir = path.app_log_dir().expect("Failed to get app log dir");
            let output_log = log_dir.join("output.log");
            output_log
                .to_str()
                .expect("Failed to convert path to string")
                .to_string() // Convert to String
        }
        None => "log/output.log".to_string(),
    };

    RollingFileAppender::builder()
        .encoder(Box::new(create_encoder()))
        .append(false)
        .build(output_path, Box::new(policy))
        .unwrap()
}

fn create_config(app: Option<&AppHandle>) -> Config {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(create_encoder()))
        .build();
    let file = create_file_appender(&app);

    Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info),
        )
        .unwrap()
}
