//! # How do a parse an input type besides `&[u8]` or `&str`?
//!
//! While historically, nom has worked mainly on `&[u8]` and `&str`, it can actually
//! use any type as input, as long as they follow a specific set of traits.
//! Those traits were developed first to abstract away the differences between
//! `&[u8]` and `&str`, but were then employed for more interesting types,
//! like [nom_locate](https://github.com/fflorent/nom_locate), a wrapper type
//! that can carry line and column information, or to parse
//! [a list of tokens](https://github.com/Rydgel/monkey-rust/blob/master/lib/parser/mod.rs).
//!
//! ## Implementing a custom type
//!
//! Let's assume we have an input type we'll call `MyInput`. `MyInput` is a sequence of `MyItem` type.
//! The goal is to define nom parsers with this signature: `MyInput -> IResult<MyInput, Output>`.
//!
//! ```rust,ignore
//! fn parser(i: MyInput) -> IResult<MyInput, Output> {
//!     tag("test")(i)
//! }
//! ```
//!
//! Here are the traits we have to implement for `MyInput`:
//!
//! | trait | usage |
//! |---|---|
//! | [AsBytes][crate::AsBytes] |Casts the input type to a byte slice|
//! | [Compare][crate::Compare] |Character comparison operations|
//! | [ExtendInto][crate::ExtendInto] |Abstracts something which can extend an `Extend`|
//! | [FindSubstring][crate::FindSubstring] |Look for a substring in self|
//! | [FindToken][crate::FindToken] |Look for self in the given input stream|
//! | [InputIter][crate::InputIter] |Common iteration operations on the input type|
//! | [InputLength][crate::InputLength] |Calculate the input length|
//! | [InputTake][crate::InputTake] |Slicing operations|
//! | [InputTakeAtPosition][crate::InputTakeAtPosition] |Look for a specific token and split at its position|
//! | [Offset][crate::Offset] |Calculate the offset between slices|
//! | [ParseTo][crate::ParseTo] |Used to integrate `&str`'s `parse()` method|
//! | [Slice][crate::Slice] |Slicing operations using ranges|
//!
//! Here are the traits we have to implement for `MyItem`:
//!
//! | trait | usage |
//! |---|---|
//! | [AsChar][crate::AsChar] |Transforms common types to a char for basic token parsing|