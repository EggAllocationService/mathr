use std::time::Instant;

use clap::Parser;
use libmapper_rs::device::Device;
use meval::Expr;


#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    devname: Option<String>,

    #[arg(short='n', long)]
    signame: Option<String>,

    #[arg(short='r', long)]
    rate: Option<u32>,

    expression: String
}

fn main() {
    let args = CliArgs::parse();
    let device_name = args.devname.unwrap_or("mathr".to_string());
    let signal_name = args.signame.unwrap_or("expr".to_string());
    let rate = args.rate.unwrap_or(10);

    let expr: Expr = args.expression.parse().expect("Malformed expression!");
    let eval = expr.bind("t").unwrap();
    println!("Creating device {} ", device_name);
    let dev = Device::create(&device_name);
    loop {
        dev.poll_and_block(10);
        if dev.is_ready() {
            break;
        }
    }
    let mut sig = dev.create_signal::<f64>(&signal_name, libmapper_rs::constants::mpr_dir::MPR_DIR_OUT);

    println!("Ready!");
    let start = Instant::now();
    loop {
        dev.poll_and_block(rate);
        let cur = start.elapsed();
        sig.set_value_single(&eval(cur.as_millis() as f64)).unwrap();
    }

}
