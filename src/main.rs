use wordy::Args;

fn main() {
    println!("Hello, world!");
    let args = Args::get();

    println!("args: {:?}", args);
}
