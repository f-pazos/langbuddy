use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;
use std::io::Write;
use std::iter::Map;
use std::path::PathBuf;

use anyhow::anyhow;

use crate::constants;
// use crate::content;
// use crate::filesystem::FileBackedBuffer;
// use crate::flashcard;
// use crate::flashcard::Flashcard;

/**
 * A REPLResult is used by the TopLevelREPL to mutate the state of the program.
 */
enum REPLResult {
    SIGBack,             // Pop out of the current subroutine.
    SIGQuit,             // Terminate the program.
    Transition(StateId), // Transition the current routine to another state.
    Err(anyhow::Error),
    Ok,
}

type RoutineId = u32;
type StateId = u32;

/**
 * A REPLState is any node that handles mutations of program state that interact
 * with the user.
 */
trait REPLState {
    fn state_name(&self) -> String;
    fn prompt(&self) -> String;
    fn handle_input(&self, input: String) -> REPLResult;
}

/**
 * A REPLRoutine represents a thread for user interaction. The TopLevelREPL is
 * itself a REPLRoutine. Routines contain a set of rules by which the current
 * thread of execution advances. Routines may spinoff child subroutines, exit
 * the program, or may transition the active REPLState of the program. The
 * TopLevelREPL is its own parent and child.
 */
struct REPLRoutine {
    name: String,
    active_id: StateId,
    states: HashMap<StateId, Box<dyn REPLState>>,
}
impl REPLRoutine {
    fn active_state(&self) -> &Box<dyn REPLState> {
        return &self.states[&self.active_id];
    }
    fn handle_input(&self, input: String) -> REPLResult {
        return self.active_state().handle_input(input);
    }
    fn transition_state(&mut self, state: StateId) -> anyhow::Result<()> {
        if !self.states.contains_key(&state) {
            return Err(anyhow!("routine {} does not {state}", self.name));
        };
        self.active_id = state;
        Ok(())
    }
}

/**
 * The TopLevelREPL represents the main interactive thread of the program. It
 * manages the user routines.
 */
pub struct TopLevelREPL {
    active_routine: RoutineId,
    routines: HashMap<RoutineId, REPLRoutine>,
}

impl TopLevelREPL {
    pub fn new() -> anyhow::Result<TopLevelREPL> {
        let mut map = HashMap::new();
        map.insert(0, Self::new_top_level_routine());
        Ok(Self {
            active_routine: 0,
            routines: map,
        })
    }

    fn new_top_level_routine() -> REPLRoutine {
        let mut states: HashMap<StateId, Box<dyn REPLState>> = HashMap::new();
        states.insert(0, Box::new(TopLevelREPLState));
        REPLRoutine {
            name: "top level routine".to_string(),
            active_id: 0,
            states,
        }
    }

    fn active_routine(&self) -> &REPLRoutine {
        return &self.routines[&self.active_routine];
    }

    fn active_routine_mut(&mut self) -> &mut REPLRoutine {
        self.routines.get_mut(&self.active_routine).unwrap()
    }

    /**
     * do_repl iterates the state of the program. It presents the current
     * state to the user, takes input, and passes the input along to the
     * necessary states.
     */
    pub fn do_repl(&mut self) -> anyhow::Result<()> {
        loop {
            let prompt = self.active_routine().active_state().prompt();
            println!("{prompt}");
            let input = self.take_input()?;
            let result = self.active_routine().handle_input(input);
            self.handle_result(&result)?;
        }
    }

    /**
     * handle_result
     */
    fn handle_result(&mut self, result: &REPLResult) -> anyhow::Result<()> {
        match result.to_owned() {
            REPLResult::SIGBack => println!("SIGback"),
            REPLResult::SIGQuit => println!("SIGquit"),
            REPLResult::Transition(id) => {
                self.active_routine_mut().transition_state(*id)?;
            }
            REPLResult::Err(error) => return Err(anyhow!(error.to_string())),
            REPLResult::Ok => return Ok(()),
        }
        Ok(())
    }

    /**
     * take_input takes user input.
     */
    fn take_input(&mut self) -> anyhow::Result<String> {
        io::stdout().flush()?;
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        Ok(s.trim().to_string())
    }
}

struct TopLevelREPLState;
impl REPLState for TopLevelREPLState {
    fn state_name(&self) -> String {
        return "Top Level State".into();
    }

    fn prompt(&self) -> String {
        return "This is the top level state.".into();
    }

    fn handle_input(&self, input: String) -> REPLResult {
        match input.as_str() {
            "quit" => REPLResult::SIGQuit,
            "back" => REPLResult::SIGBack,
            "hello" => {
                println!("hello yourself");
                REPLResult::Ok
            }
            _ => REPLResult::Ok,
        }
    }
}
