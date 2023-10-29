mod display;

fn main() -> std::io::Result<()> {
    let (sender, join_handle) = display::Display::start(std::io::stdout());

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer)?;
    sender.send(None).expect("Failed to send empty");
    join_handle.join().expect("Failed to join thread");

    Ok(())
}
