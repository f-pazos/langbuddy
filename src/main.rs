// mod control;
// use control::TopLevelControl;

mod terminal;
// mod content;

mod orchestrator;
use orchestrator::Orchestrator;

mod control;
// mod flashcard;
// mod flashcard_deck;

fn main() -> anyhow::Result<()> {
    let mut orchestrator = orchestrator::Orchestrator::new();
    loop {
        match orchestrator.repl() {
            Err(x) => {
                println!("encountered error {x}, quitting.");
                return Ok(());
            }
            _ => (),
        }
    }
}
