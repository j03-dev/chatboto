pub mod gemini;
pub mod mistral;

#[macro_export]
macro_rules! perform_task {
    ($m:expr,$api_key:expr, $prompt:expr, $history:expr) => {{
        let model = Box::leak(Box::new($m));
        model.set_api_key($api_key);
        iced::Task::perform(model.ask($prompt, $history), |result| {
            crate::Message::AIRespond(result.unwrap_or_else(|err| err.to_string()))
        })
    }};
}
