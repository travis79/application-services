/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::process::Command;

pub fn main() {
    #[cfg(feature = "uniffi-bindings")]
    uniffi_build::generate_scaffolding("./src/nimbus.udl").unwrap();
    // println!("cargo:rerun-if-changed=./src/nimbus.udl");

    Command::new("glean_parser")
        .args(&[
            "translate",
            "-f",
            "\"rust\"",
            "-o",
            "./src/",
            "./metrics.yaml",
        ])
        .status()
        .unwrap();
    // println!("cargo:rerun-if-changed=./metrics.yaml");
}
