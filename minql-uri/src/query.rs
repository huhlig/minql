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

/// Query
///
/// Per [Wikipedia](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier):
/// > An optional query component preceded by a question mark (?), consisting of a query string of
/// > non-hierarchical data. Its syntax is not well-defined, but by convention is most often a
/// > sequence of attribute–value pairs separated by a delimiter.
///
/// ```bnf
/// query     ::= parameter [ [';' | '&'] parameter]
/// parameter ::= key '=' value
/// key       ::= non-reserved
/// value     ::= non-reserved
/// ```
#[derive(Debug)]
pub struct Query<'str> {
    /// Raw Unparsed Query String
    pub string: &'str str,
    /// Query Parameters Split by `&` or ';' and parameters split by `=`
    pub parameters: Vec<(&'str str, Vec<&'str str>)>,
}
