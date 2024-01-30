use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd;

impl Cmd {
    pub fn run() {
        let v = hcnet_strkey::VERSION;
        println!("hcnet-strkey {} ({})", v.pkg, v.rev);
    }
}
