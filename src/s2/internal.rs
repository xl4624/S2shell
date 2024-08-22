// Copyright 2005 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS-IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// Original Author: ericv@google.com (Eric Veach)

pub const SWAP_MASK: i32 = 0x01;
pub const INVERT_MASK: i32 = 0x02;

#[rustfmt::skip]
pub const POS_TO_IJ: [[i32; 4]; 4] = [
  // 0  1  2  3
    [0, 1, 3, 2], // canonical order:    (0,0), (0,1), (1,1), (1,0)
    [0, 2, 3, 1], // axes swapped:       (0,0), (1,0), (1,1), (0,1)
    [3, 2, 0, 1], // bits inverted:      (1,1), (1,0), (0,0), (0,1)
    [3, 1, 0, 2], // swapped & inverted: (1,1), (0,1), (0,0), (1,0)
];

pub const POS_TO_ORIENTATION: [i32; 4] = [SWAP_MASK, 0, 0, INVERT_MASK + SWAP_MASK];
