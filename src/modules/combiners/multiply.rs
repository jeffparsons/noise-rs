// Copyright 2016 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that outputs the product of the two output values from two source
/// modules.
pub struct Multiply<Source1, Source2> {
    /// Outputs a value.
    pub source1: Source1,

    /// Outputs a value.
    pub source2: Source2,
}

impl<Source1, Source2> Multiply<Source1, Source2> {
    pub fn new(source1: Source1, source2: Source2) -> Multiply<Source1, Source2> {
        Multiply {
            source1: source1,
            source2: source2,
        }
    }
}

impl<Source1, Source2, T, U> NoiseModule<T> for Multiply<Source1, Source2>
    where Source1: NoiseModule<T, Output = U>,
          Source2: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        self.source1.get(point) * self.source2.get(point)
    }
}
