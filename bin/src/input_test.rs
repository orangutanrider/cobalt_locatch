use locatch_macro::*;
use locatch_lib::*;
use locatch_process::*;

use clap::*;
use serde::*;

fn main() {    
    let cli = Cli::parse();
    println!("{:?}", cli.input);

    // Recieve input
    let input = match input_reception(&cli.input) {
        Ok(ok) => ok,
        Err(_) => {
            println!("Error with reception of input");
            return;
        },
    };
}

fn print_serial_input(input: SerialInput) {
    let marco_print: Result<(), JsonError> = match input.marco {
        Some(marco) => {
            println!("Printing macro...");
            print_ser_request(marco)
        },
        None => Ok(()),
    };

    match marco_print {
    Ok(_) => todo!(),
    Err(_) => todo!(),
    }

}

fn print_ser_request(request: SerialRequest) -> Result<(), JsonError> {
    let serialized = match request.to_json() {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    println!("{}", serialized);
    return Ok(())
}