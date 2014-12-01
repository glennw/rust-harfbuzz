// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// FIXME: This is a huge hack to find the static version of the library
// instead of the shared. It looks in a very specific place that only has
// relevance to servo.

#[cfg(target_os = "linux")]
#[link(name = "harfbuzz")]
#[link(name = "glib-2.0")]
#[link(name = "stdc++")]
extern { }

#[cfg(any(target_os = "macos", target_os = "android"))]
#[link(name = "harfbuzz", kind = "static")]
#[link(name = "stdc++")]
extern { }
