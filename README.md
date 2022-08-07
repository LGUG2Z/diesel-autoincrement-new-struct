# diesel-autoincrement-new-struct

Stop copying and pasting structs with the `id` field removed just so you can
insert objects into tables with autoincrementing primary keys.

## Motivation

I mean really, who wants to do all that copying and pasting? Who wants to keep
attributes and documentation in sync between two otherwise identical structs?

## Examples

First, the ugly example, in which you wrap your whole struct definition in
a macro:

```rust
use diesel_autoincrement_new_struct::diesel_new;
use diesel::prelude::*;

table! {
    users(id) {
        id -> Integer,
        name -> Text,
    }
}

diesel_new! {
    #[derive(Debug, Clone, Queryable, AsChangeset)]
    #[diesel(table_name = users)]
    /// This is a user
    pub struct User {
        /// This is the ID of the user
        id: i32,
        /// This is the name of the user
        name: String
    }
}

// The code below gets generated by `diesel_new!`

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[derive(Insertable)]
#[diesel(table_name = users)]
/// This is a user
pub struct NewUser {
    /// This is the name of the user
    name: String
}
```

Pretty neat, right? But we can do better. Check this out.

```rust
#[macro_use]
extern crate macro_rules_attribute;

use diesel::prelude::*;

attribute_alias! {
    #[apply(New!)] = #[macro_rules_derive(diesel_autoincrement_new_struct::diesel_new)];
}

table! {
    users(id) {
        id -> Integer,
        name -> Text,
    }
}

#[apply(New!)]
#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = users)]
/// This is a user
pub struct User {
    /// This is the ID of the user
    id: i32,
    /// This is the name of the user
    name: String
}

// The code below gets generated by `#[apply(New!)]`

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[derive(Insertable)]
#[diesel(table_name = users)]
/// This is a user
pub struct NewUser {
    /// This is the name of the user
    name: String
}
```

Much better, ne?

This whole idea came about after finding the excellent
[`macro_rules_attribute`](https://github.com/danielhenrymantilla/macro_rules_attribute-rs)
crate by [Daniel Henry-Mantilla](https://github.com/danielhenrymantilla),
aka my personal Rust macro hero.

If you want to use this crate with the `#[apply]` attribute as in the second
example, make sure you add `macro_rules_attribute` to your `Cargo.toml`:

```toml
macro_rules_attribute = "0.1"
```

And add this at the top of your `lib.rs` or `main.rs` file:

```rust
#[macro_use]
extern crate macro_rules_attribute;

attribute_alias! {
    #[apply(New!)] = #[macro_rules_derive(diesel_autoincrement_new_struct::diesel_new)];
}
```

Then, you can import `crate::New` in whichever file you want and apply it to your structs with the `#[apply]` attribute.

## Notes

- This crate doesn't re-export [Diesel](https://github.com/diesel-rs/diesel), so make sure you have `use diesel::prelude::*;` or `use diesel::Insertable;` in whichever files you use this macro
- This crate requires at least whichever version or revision of Diesel where the `#[diesel(table_name = ...)]` attribute stopped taking a double quoted string

The `#[apply]` attribute should always be the topmost attribute above a struct,
unless the struct that you want to use it on is also deriving `Identifiable`.
If that is the case, you should have that derive been the topmost attribute
above the struct so that it is excluded when generating the `NewStruct`, becuase
obviously, without an `id`, it won't be `Identifiable`:

```rust
#[derive(Identifiable)]
#[apply(New!)]
#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = users)]
/// This is a user
pub struct User {
    /// This is the ID of the user
    id: i32,
    /// This is the name of the user
    name: String
}
```
