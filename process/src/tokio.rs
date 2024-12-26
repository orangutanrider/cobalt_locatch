use tokio::runtime::Runtime;

use locatch_macro::*;

pub fn new_async_runtime(thread_count: &Option<usize>, stack_size: &Option<usize>) -> Result<Runtime, IOError> { 
    let mut runtime = tokio::runtime::Builder::new_multi_thread();

    if let Some(thread_count) = thread_count {
        runtime.worker_threads(*thread_count);
    }

    if let Some(stack_size) = stack_size {
        runtime.thread_stack_size(*stack_size);
    }

    runtime.enable_all();

    return runtime.build();
}