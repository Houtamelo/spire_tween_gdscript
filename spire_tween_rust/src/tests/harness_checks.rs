//! Deliberate failure fixtures, selected only with --runner-self-test.
//! The external real-engine check verifies actual aggregate process exit codes.
use std::time::Duration;

use godot::prelude::*;

use super::util::*;

pub(super) async fn run_case(parent: &mut Gd<Node>, name: &str, timeout: Duration) -> usize {
    let tree = parent.get_tree();
    let case = name.to_owned();
    let run = start_test(format!("runner-self-test::{case}"), move || {
        if case == "setup_panic" {
            panic!("deliberate setup failure");
        }
        match case.as_str() {
            "pass" => Box::pin(async {}),
            "async_pass" => Box::pin(async {
                next_frame().await;
            }),
            "async_panic" => Box::pin(async {
                next_frame().await;
                panic!("deliberate asynchronous failure");
            }),
            "callback_panic" => {
                assert_upon_emission(Signal::from_object_signal(&tree, "process_frame"), || {
                    assert_eq!(1, 2, "deliberate callback assertion");
                });
                Box::pin(async {
                    next_frame().await;
                    next_frame().await;
                })
            }
            "timeout" => Box::pin(std::future::pending()),
            "paused_timeout" => {
                let mut tree = tree;
                tree.set_pause(true);
                Box::pin(std::future::pending())
            }
            _ => panic!("Unknown runner self-test: {case}"),
        }
    });
    usize::from(!wait_for_task(run, timeout).await)
}
