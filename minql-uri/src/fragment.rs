//
// Copyright 2024 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

/// # URI Fragment
///
/// Per [Wikipedia](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier):
/// > An optional fragment component preceded by a hash (#). The fragment contains a fragment
/// > identifier providing direction to a secondary resource, such as a section heading in an
/// > article identified by the remainder of the URI. When the primary resource is an HTML document,
/// > the fragment is often an id attribute of a specific element, and web browsers will scroll this
/// > element into view.
///
/// ```bnf
/// fragment ::= <non-reserved>
/// ```
///
#[derive(Debug)]
pub struct Fragment<'str>(pub &'str str);
