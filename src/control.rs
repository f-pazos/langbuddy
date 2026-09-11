use std::collections::HashMap;
use std::io;
use std::io::Write;

use anyhow::anyhow;

type Input = String;

use crate::terminal;

/**
 * An InteractiveRoutine provides an abstraction to encapsulate user
 * interaction. It defines the minimum requirements for interactivity.
 */
pub trait InteractiveRoutine {
    // Each InteractiveRoutine implementation specifies its own Signal type.
    // This encapsulates the values that can be communicated to the Routine
    // externally.
    // type Signal;

    // Handle signals created by the control surface and update internal state.
    fn process_inputs(&mut self, inputs: terminal::Inputs);

    // Returns signals for the higher level program to handle.
    fn poll_signals(&self) -> &RoutineSignal;

    // Render internal state as content to be displayed.
    fn render_content(&self) -> terminal::Content;
}

type RoutineSignal = Vec<Signals>;

/**
 * A Signals communicates the execution signal to the top level.
 */
pub enum Signals {
    SIGTerminate,          // Terminate the routine.
    SIGErr(anyhow::Error), // The routine encountered an error.
}

//**
//  * A ControlState is an entity that represents interaction with the user. The
//  * program only has a single ControlState running at any time. ControlStates
//  * receive inputs and return outputs for the program view to display.
//  */
// pub trait InteractionNode<R: InteractiveRoutine> {
//     type Context;
//     type Input;
//     type Output;

//     fn initialize(ctx: Self::Context) -> Self;
//     fn handle_input(&self, input: Self::Input) -> R::Signal;
//     fn output(&self) -> Self::Output;
// }

// /**
//  * A ControlRoutine
//  */
// pub trait ControlRoutine<S: ControlState> {
//     fn name(&self) -> String;
//     fn states(&self) -> HashMap<S::StateId, S>;

//     fn advance_routine(&self, input: String) -> RoutineSignal;
// }

// enum BasicStateResponse<Id> {
//     Transition(Id),
//     Ok,
//     Err,
// }
// pub struct BasicState;

// impl ControlState for BasicState {}

// enum StateResponses {
//     Transition(u32),
// }

// /**
//  * A Routine represents content that takes control of a view. It provides a
//  * convenient way to define interactive content.
//  */
// pub trait Routine<S: ControlState> {
//     type StateId = S::StateId;

//     /** Human readable routine name. Should be all lowercase. */
//     fn name(&self) -> String;
//     fn states(&self) -> HashMap<StateId, S>;

//     /** evolve_routine handles user input and returns the StateId that the
//      * routine should now display.**/
//     fn evolve_routine(&self, input: UserInput) -> StateId;
// }

// struct BasicState {
//     name: String,
//     prompt: String,
//     transitions_to: u32,
// }

// impl State for BasicState {
//     type StateId = u32;
//     fn state_name(&self) -> String {"basic state name".to_string()}
//     fn prompt(&self) -> String {"prompt".to_string()}
//     fn handle_input(&self, input: String) -> ControlResult{
//         ControlResult::Transition(self.transitions_to)
//     }
// }

// /**
//  * A BasicRoutine represents a thread for user interaction. A ControlRoutine manages
//  * a collection of ControlStates to form a cohesive user experience. Routines may
//  * spinoff child subroutines, exit the program, or may transition the active
//  * ControlState of the program.
//  */
// struct BasicRoutine<Id> {
//     name: String,
//     active_state: Id,
//     states: HashMap<Id, Box<dyn ControlState>>,
// }

// impl Routine<BasicState> for BasicRoutine<String> {
//     type StateId = String;
//     fn name(&self) -> String {
//         "basic routine".to_string()
//     }
//     fn states(&self) -> HashMap<StateId,
// }

// impl BasicRoutine {
//     fn active_state(&self) -> &Box<dyn ControlState> {
//         return &self.states()[&self.active_state];
//     }
//     fn handle_input(&self, input: String) -> ControlResult {
//         return self.active_state().handle_input(input);
//     }
//     fn transition_state(&mut self, state: StateId) -> anyhow::Result<()> {
//         if !self.states().contains_key(&state) {
//             return Err(anyhow!("routine {} does not {state}", self.name));
//         };
//         self.active_state = state;
//         Ok(())
//     }
// }

// /**
//  * The TopLevelControl represents the main interactive thread of the program. It
//  * manages the user routines.
//  */
// pub struct TopLevelControl {
//     active_routine: RoutineId,
//     routines: HashMap<RoutineId, MainControlRoutine>,
// }

// impl TopLevelControl {
//     pub fn new() -> anyhow::Result<TopLevelControl> {
//         let mut map = HashMap::new();
//         map.insert(0, Self::new_top_level_routine());
//         Ok(Self {
//             active_routine: 0,
//             routines: map,
//         })
//     }

//     fn new_top_level_routine() -> MainControlRoutine {
//         let mut states: HashMap<StateId, Box<dyn ControlState>> = HashMap::new();
//         states.insert(0, Box::new(TopLevelControlState));
//         MainControlRoutine {
//             name: "top level routine".to_string(),
//             active_state: 0,
//             states,
//         }
//     }

//     fn active_routine(&self) -> &MainControlRoutine {
//         return &self.routines[&self.active_routine];
//     }

//     fn active_routine_mut(&mut self) -> &mut MainControlRoutine {
//         self.routines.get_mut(&self.active_routine).unwrap()
//     }

//     /**
//      * do_repl iterates the state of the program. It presents the current
//      * state to the user, takes input, and passes the input along to the
//      * necessary states.
//      */
//     pub fn do_repl(&mut self) -> anyhow::Result<()> {
//         loop {
//             let prompt = self.active_routine().active_state().prompt();
//             println!("{prompt}");
//             let input = self.take_input()?;
//             let result = self.active_routine().handle_input(input);
//             self.handle_result(&result)?;
//         }
//     }

//     /**
//      * handle_result
//      */
//     fn handle_result(&mut self, result: &ControlResult) -> anyhow::Result<()> {
//         match result.to_owned() {
//             ControlResult::SIGBack => println!("SIGback"),
//             ControlResult::SIGQuit => println!("SIGquit"),
//             ControlResult::Transition(id) => {
//                 self.active_routine_mut().transition_state(*id)?;
//             }
//             ControlResult::Err(error) => return Err(anyhow!(error.to_string())),
//             ControlResult::Ok => return Ok(()),
//         }
//         Ok(())
//     }

//     /**
//      * take_input takes user input.
//      */
//     fn take_input(&mut self) -> anyhow::Result<String> {
//         io::stdout().flush()?;
//         let mut s = String::new();
//         io::stdin().read_line(&mut s)?;
//         Ok(s.trim().to_string())
//     }
// }

// struct TopLevelControlState;
// impl ControlState for TopLevelControlState {
//     type StateId = u32;
//     fn state_name(&self) -> String {
//         return "Top Level State".into();
//     }

//     fn prompt(&self) -> String {
//         return "This is the top level state.".into();
//     }

//     fn handle_input(&self, input: String) -> ControlResult {
//         match input.as_str() {
//             "quit" => ControlResult::SIGQuit,
//             "back" => ControlResult::SIGBack,
//             "hello" => {
//                 println!("hello yourself");
//                 ControlResult::Ok
//             }
//             _ => ControlResult::Ok,
//         }
//     }
// }
