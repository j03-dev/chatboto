pub mod gemini;
pub mod mistral;

#[macro_export]
macro_rules! perform_task {
    ($m:expr,$api_key:expr, $prompt:expr, $history:expr) => {{
        let model = Box::leak(Box::new($m));
        model.set_api_key($api_key);
        iced::Task::perform(model.ask($prompt, $history), |result| {
            $crate::Message::AIRespond(result.unwrap_or_else(|err| err.to_string()))
        })
    }};
}

#[macro_export]
macro_rules! impl_model {
    ($trait:ident, $struct:ident, $version:expr) => {
        #[derive(Debug, Clone, Default, PartialEq, Eq)]
        pub struct $struct {
            api_key: Option<String>,
        }

        impl $trait for $struct {
            const VERSION: &'static str = $version;

            fn api_key(&self) -> &str {
                &self.api_key.as_ref().unwrap()
            }

            fn set_api_key(&mut self, key: &str) {
                self.api_key = Some(key.into());
            }
        }
    };
}
