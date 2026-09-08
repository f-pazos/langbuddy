use anyhow::Result;
use scraper::error::SelectorErrorKind::QualRuleInvalid;
use std::{collections, io};

use crate::control::InteractiveRoutine;

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
}

pub struct DefaultRoutine;
impl InteractiveRoutine for DefaultRoutine {
    fn process_signals(&self, signals: Vec<String>) -> anyhow::Result<()> {
        Ok(())
    }

    fn poll_signals(&self) -> Vec<crate::control::RoutineSignal> {
        vec![]
    }

    fn poll_content(&self) -> crate::terminal::Content {
        "default_routine".to_owned().into()
    }
}

type RoutineID = u64;

impl Orchestrator {
    pub fn new() -> Self {
        let mut orchestrator = Self {
            routine_id: 0,
            routines: collections::HashMap::new(),
            active_id: 0,
        };

        let default_id = orchestrator.register_routine(DefaultRoutine);
        orchestrator.active_id = default_id;

        return orchestrator;
    }

    /**
     * Step through the REPL.
     */
    pub fn repl(&self) -> anyhow::Result<()> {
        let content = self.routines[&self.active_id].poll_content();
        println!("{}", content.text);
        let input = input()?;

        Ok(())
    }

    pub fn register_routine<R: InteractiveRoutine + 'static>(&mut self, routine: R) -> RoutineID {
        self.routine_id += 1;
        self.routines.insert(self.routine_id, Box::new(routine));
        return self.routine_id;
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

fn input() -> anyhow::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    return Ok(s);
}
