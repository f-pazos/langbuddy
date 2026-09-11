mod terminal;

mod orchestrator;
use orchestrator::Orchestrator;

mod top_level_routine;

mod control;

fn main() -> anyhow::Result<()> {
    let mut orchestrator = orchestrator::Orchestrator::new();
    orchestrator.orchestrate()
}
