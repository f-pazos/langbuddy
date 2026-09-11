use crate::{
    control::{self, InteractiveRoutine, Signals},
    terminal,
};

pub struct TopLevelRoutine {
    text: String,
    signals: Vec<control::Signals>,
}

impl Default for TopLevelRoutine {
    fn default() -> Self {
        Self {
            text: "top_level_routine".to_string(),
            signals: Vec::new(),
        }
    }
}

impl InteractiveRoutine for TopLevelRoutine {
    fn process_inputs(&mut self, inputs: terminal::Inputs) {
        self.text = inputs.content;

        if self.text == "quit\n" {
            self.signals = vec![control::Signals::SIGTerminate]
        }
    }

    fn poll_signals(&self) -> &Vec<control::Signals> {
        return &self.signals;
    }

    fn render_content(&self) -> terminal::Content {
        self.text.to_owned().into()
    }
}
