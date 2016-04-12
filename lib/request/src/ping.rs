//  rpc-perf - RPC Performance Testing
//  Copyright 2015 Twitter, Inc
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

#[cfg(feature = "unstable")]
extern crate test;

/// ping request
///
/// # Example
/// ```
/// # use rpcperf_request::ping::*;
///
/// assert_eq!(ping(), "PING\r\n");
pub fn ping() -> String {
    "PING\r\n".to_owned()
}

#[cfg(feature = "unstable")]
#[bench]
fn ping_benchmark(b: &mut test::Bencher) {
    b.iter(|| ping());
}
