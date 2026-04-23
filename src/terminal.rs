
fn field_input(&self) -> anyhow::Result<()> {
    Ok(())
    io::stdout().flush()?;
    let word = input();

    // if word.is_err() {
    //     return Err(anyhow!("problem receiving input: {}", word.unwrap_err()));
    // };

    // let word = word.unwrap();
    // let word = word.trim();

    // if word == "save" {
    //     return Ok(UserInput::Command(Command::Save));
    // }

    // return Ok(UserInput::Word(word.to_string()));
} 

fn input() -> anyhow::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    return Ok(s);
}