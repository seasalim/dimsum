# dimsum - A simple multi-dimensional array library for Rust

## Overview

dimsum is a simple, easy-to-use multi-dimensional array library in Rust. It allows an arbitrary number of dimensions. It also chooses to use explicit get and set functions instead of an Index trait in order to facilitate out-of-bounds error handling.

## Usage Example

```
let mut md = MultiDim::<u32>::new(&[2, 2, 2], 0);
md.set(&[0, 1, 1], 10).unwrap();
assert_eq!(10, *md.get(&[0, 1, 1]).unwrap());
```

## Author(s)

Salim Alam

## License

Source code is released under the Apache 2.0 license as follows:

Copyright 2017 Salim Alam

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
