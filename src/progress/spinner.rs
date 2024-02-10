use indicatif::{ProgressBar, ProgressStyle};
use tokio::{sync::oneshot, time::Duration};

pub struct Spinner {
    handle: tokio::task::JoinHandle<()>,
    finish_signal: oneshot::Sender<String>,
}

impl Spinner {
    pub fn start(text: impl Into<String>) -> Self {
        let text = text.into();

        let load_template = format!("{{spinner}} {}", text);
        let finish_template = format!("{{msg}} {}", text);

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template(load_template.as_str())
                .expect("Invalid template for spinner."),
        );

        let (tx, mut rx) = oneshot::channel();

        let handle = tokio::task::spawn(async move {
            loop {
                spinner.tick();
                tokio::time::sleep(Duration::from_millis(100)).await;
                if let Ok(message) = rx.try_recv() {
                    spinner.set_style(
                        ProgressStyle::default_spinner()
                            .template(finish_template.as_str())
                            .expect("Invalid template for spinner."),
                    );
                    spinner.finish_with_message(message);
                    break;
                }
            }
        });

        Spinner {
            handle,
            finish_signal: tx,
        }
    }

    pub async fn finish(self, message: impl Into<String>) {
        let _ = self.finish_signal.send(message.into());
        if let Err(_) = self.handle.await {}
    }
}
