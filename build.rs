use clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => {
            println!("Hellooo");
            return;
        }
        Some(outdir) => outdir,
    };

    println!("{:?}", outdir);

    let mut app = build_cli();
    app.gen_completions("tmc", Shell::Bash, &outdir);
    app.gen_completions("tmc", Shell::Fish, &outdir);
    app.gen_completions("tmc", Shell::PowerShell, &outdir);
    app.gen_completions("tmc", Shell::Zsh, &outdir);
}
