// Logging planned

/* 
use locatch_macro::*;

use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
/// Serial file input struct.  
pub struct List {
    #[serde(alias = "macro")]
    pub marco: Option<SerialRequestMacro>,
    pub tickets: Vec<SerialRequest>,
}
impl_from_json!(List);
impl List {
    /// Apply the macro to the requests (if there is a macro)
    pub fn apply_macro(&mut self) {
        let Some(marco) = &self.marco else {
            // There is no macro to apply
            return;
        };

        for request in self.tickets.iter_mut() {
            request.apply_macro(marco);
        }
    }

    // Thereotically more performant.
    // Instead of cloning state at each step of the iteration, the entire vec is simply cloned and then values are fed in as state.
    // Un-tested.
    pub fn apply_macro_vec_clone(&mut self) {
        let Some(marco) = &self.marco else {
            // There is no macro to apply
            return;
        };

        let mut state_iter = self.tickets.clone().into_iter();
        for request in self.tickets.iter_mut() {
            let Some(state) = state_iter.next() else {
                continue; // should never happen, log?
            };

            request.apply_macro_with(state, marco);
        }
    }

    /// Apply macro onto requests in parallel
    /// Un-implemented
    pub fn apply_macro_par() {
        todo!()
    }
}
*/