// Logging planned

use locatch_macro::{impl_from_str_json, impl_from_string_json};
use serde::Deserialize;

use super::{
    filename_macro::{apply_filename_macro, FilenameMacro}, 
    ticket::Ticket, 
    ticket_macro::{apply_ticket_macro, TicketMacro}
};

#[derive(Deserialize)]
pub struct List {
    pub filename_macro: Option<FilenameMacro>,
    pub ticket_macro: Option<TicketMacro>,
    pub tickets: Vec<Ticket>,
}
impl_from_str_json!(List);
impl_from_string_json!(List);
impl List {
    pub fn apply_internal_macros(&mut self) {
        self.apply_internal_ticket_macro();
        self.apply_internal_filename_macro();
    }

    pub fn apply_filename_macro(&mut self, filename_macro: &FilenameMacro) {
        apply_filename_macro(filename_macro, &mut self.tickets);
    }

    pub fn apply_ticket_macro(&mut self, ticket_macro: &TicketMacro) {
        for ticket in self.tickets.iter_mut() {
            apply_ticket_macro(ticket_macro, ticket);
        }
    }

    fn apply_internal_filename_macro(&mut self) {
        let filename_macro = match &self.filename_macro {
            Some(val) => val,
            None => return,
        };

        apply_filename_macro(filename_macro, &mut self.tickets);
    }

    fn apply_internal_ticket_macro(&mut self) {
        let ticket_macro = match &self.ticket_macro {
            Some(val) => val,
            None => return,
        };

        for ticket in self.tickets.iter_mut() {
            apply_ticket_macro(ticket_macro, ticket);
        }
    }
}
