pub use paste::paste;
/// Macro to generate a New struct for Diesel insertions without an 'id' field
///
/// All struct and field metadata is kept; documentation, serde attributes etc.
///
/// # Example:
///
/// ```rust
/// use diesel_autoincrement_new_struct::diesel_new;
/// use diesel::prelude::*;
///
/// table! {
///     users(id) {
///         id -> Integer,
///         name -> Text,
///     }
/// }
///
/// diesel_new! {
///     /// This is a user
///     #[derive(Debug, Clone, Queryable, AsChangeset)]
///     #[diesel(table_name = users)]
///     pub struct User {
///         /// This is the ID of the user
///         id: i32,
///         /// This is the name of the user
///         name: String
///     }
///  
///     // The macro will generate the following output:
///     //
///     // /// This is a user
///     // #[derive(Debug, Clone, Queryable, AsChangeset)]
///     // #[derive(Insertable)]
///     // #[diesel(table_name = users)]
///     // pub struct NewUser {
///     //    /// This is the name of the user
///     //    name: String
///     // }
/// }
/// ```
#[macro_export]
macro_rules! diesel_new {
    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $StructName:ident {
            // We wanna make sure we don't catch the ID struct in the repetition
            $(#[$_id_meta:meta])*
            $_id_field_vis:vis id : $_id_type:ty,
            // Here is the repetition for every field except the ID field
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
    ) => (
        $crate::paste! {
            $(#[$struct_meta])*
            #[derive(diesel::Insertable)]
            $struct_vis struct [< New $StructName >] {
                $(
                    $(#[$field_meta])*
                    $field_vis $field_name: $field_ty,
                )*
            }
        }
    );
}

#[cfg(test)]
mod tests {
    use diesel::debug_query;
    use diesel::prelude::*;

    table! {
        users(id) {
            id -> Integer,
            name -> Text,
        }
    }

    super::diesel_new! {
        #[derive(Debug, Clone, Queryable, AsChangeset)]
        #[diesel(table_name = users)]
        pub struct User {
            id: i32,
            pub name: String
        }
    }

    #[test]
    fn it_generates_a_new_struct() {
        NewUser {
            name: String::from("Ferris"),
        };
    }

    #[test]
    fn it_can_create_an_insert_statement() {
        let query = NewUser {
            name: String::from("Ferris"),
        }
        .insert_into(users::table);

        assert_eq!(
            r#"INSERT INTO `users` (`name`) VALUES (?) -- binds: ["Ferris"]"#,
            debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
        );
    }
}
