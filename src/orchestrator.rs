use anyhow::Result;
use scraper::error::SelectorErrorKind::QualRuleInvalid;
use std::{collections, io};

use crate::{control::InteractiveRoutine, terminal};

/**
 * The Orchestrator is the top level thread of execution for the program. It
 * coordinates communication between the active routine, input and output
 * methods, handles transitions between states and routines, and manages
 * persistance across program invocations.
 */
pub struct Orchestrator {
    routine_id: u64,
    routines: collections::HashMap<u64, Box<dyn InteractiveRoutine>>,
    active_id: RoutineID,
    terminal_client: terminal::TerminalClient,
}

pub struct DefaultRoutine {
    text: String,
}

impl Default for DefaultRoutine {
    fn default() -> Self {
        Self {
            text: "default_routine".to_string(),
        }
    }
}

impl InteractiveRoutine for DefaultRoutine {
    fn process_signals(&mut self, signal: String) {
        self.text = signal;
    }

    fn poll_signals(&self) -> Vec<crate::control::RoutineSignal> {
        vec![]
    }

    fn render_content(&self) -> crate::terminal::Content {
        self.text.to_owned().into()
    }
}

type RoutineID = u64;

impl Orchestrator {
    pub fn new() -> Self {
        let mut orchestrator = Self {
            routine_id: 0,
            routines: collections::HashMap::new(),
            active_id: 0,
            terminal_client: terminal::TerminalClient,
        };

        let default_id = orchestrator.register_routine::<DefaultRoutine>(Default::default());
        orchestrator.active_id = default_id;

        return orchestrator;
    }

    /**
     * Step through the REPL.
     */
    pub fn repl(&mut self) -> anyhow::Result<()> {
        let content = self.active_routine().render_content();
        self.terminal_client.display_content(content);

        let input = self.terminal_client.poll_input();
        self.active_routine().process_signals(input);
        Ok(())
    }

    pub fn register_routine<R: InteractiveRoutine + 'static>(&mut self, routine: R) -> RoutineID {
        self.routine_id += 1;
        self.routines.insert(self.routine_id, Box::new(routine));
        return self.routine_id;
    }

    fn active_routine(&mut self) -> &mut Box<dyn InteractiveRoutine> {
        self.routines.get_mut(&self.active_id).unwrap()
    }
}

// fn field_input(&self) -> anyhow::Result<()> {
//     Ok(())
//     io::stdout().flush()?;
//     let word = input();

//     // if word.is_err() {
//     //     return Err(anyhow!("problem receiving input: {}", word.unwrap_err()));
//     // };

//     // let word = word.unwrap();
//     // let word = word.trim();

//     // if word == "save" {
//     //     return Ok(UserInput::Command(Command::Save));
//     // }

//     // return Ok(UserInput::Word(word.to_string()));
// }
