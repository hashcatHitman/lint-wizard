// SPDX-FileCopyrightText: © 2026 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate alloc;

use alloc::string;
use std::io;

use error_set::error_set;

error_set! {
    CommandOutput := {
        FromUtf8(string::FromUtf8Error),
        Io(io::Error)
    }
}
