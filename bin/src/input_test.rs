use locatch_macro::*;
use locatch_process::*;

use clap::*;

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

    print_serial_input(&input);
}

fn print_serial_input(input: &List) {
    let marco_print: Result<(), JsonError> = match &input.marco {
        Some(marco) => {
            println!("Printing macro...");
            print_ser_request(marco)
        },
        None => {
            println!("Macro is empty");
            Ok(())
        },
    };

    match marco_print {
        Ok(_) => {/* Do nothing */},
        Err(err) => {
            println!("Failed to serialize the deserialized macro, error: \n \t{}", err);
        },
    }

    println!("Printing {} requests...", input.tickets.len());
    let mut index: usize = 0;
    for request in input.tickets.iter() {
        println!("Request-{}...", index);
        match print_ser_request(request) {
            Ok(_) => {/* Do nothing */},
            Err(err) => {
                println!("Unexpected error, was unable to serialize the request for the print; The request was gained by first deserializing it, was unable to serialize data that was previously deserialized?");
                println!("Error: {}", err);
            },
        }
        
        index = index + 1;
    }
    
}

fn print_ser_request(request: &SerialRequest) -> Result<(), JsonError> {
    let serialized = match request.to_json() {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    println!("{}", serialized);
    return Ok(())
}