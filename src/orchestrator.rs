use anyhow::Result;
use scraper::error::SelectorErrorKind::QualRuleInvalid;
use std::{
    collections, io,
    process::{self, exit},
};

use crate::{
    control::{self, InteractiveRoutine},
    terminal,
    top_level_routine::TopLevelRoutine,
};

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

type RoutineID = u64;

impl Orchestrator {
    pub fn new() -> Self {
        let mut orchestrator = Self {
            routine_id: 0,
            routines: collections::HashMap::new(),
            active_id: 0,
            terminal_client: terminal::TerminalClient,
        };

        let default_id = orchestrator.register_routine::<TopLevelRoutine>(Default::default());
        orchestrator.active_id = default_id;

        return orchestrator;
    }

    pub fn orchestrate(&mut self) -> anyhow::Result<()> {
        loop {
            match self.iterate() {
                Err(x) => {
                    println!("encountered error {x}, quitting.");
                    return Ok(());
                }
                _ => (),
            }
        }
    }

    /**
     * Step through the REPL.
     */
    fn iterate(&mut self) -> anyhow::Result<()> {
        let content = self.active_routine().render_content();
        self.terminal_client.display_content(content);

        let input = self.terminal_client.poll_input();
        self.active_routine()
            .process_inputs(terminal::Inputs { content: input });

        for signal in self.active_routine().poll_signals() {
            match signal {
                control::Signals::SIGTerminate => {
                    process::exit(0);
                }
                _ => {
                    todo!();
                }
            }
        }

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
