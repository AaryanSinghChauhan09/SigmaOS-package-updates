//! Integration tests for SigmaOS-package-updates.

use sigmaos_package_updates_lib::{Component, ComponentLifecycle};

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
