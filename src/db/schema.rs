// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel::pg::sql_types::*;

    book_categories (book_id, category_id) {
        book_id -> Int8,
        category_id -> Int8,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::pg::sql_types::*;

    book_tags (book_id, tag_id) {
        book_id -> Int8,
        tag_id -> Int8,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::pg::sql_types::*;

    books (id) {
        id -> Int8,
        #[max_length = 20]
        isbn -> Nullable<Varchar>,
        #[max_length = 200]
        title -> Varchar,
        #[max_length = 100]
        author -> Varchar,
        #[max_length = 100]
        publisher -> Nullable<Varchar>,
        publication_date -> Nullable<Date>,
        page_count -> Nullable<Int4>,
        #[max_length = 255]
        cover_image -> Nullable<Varchar>,
        description -> Nullable<Text>,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::pg::sql_types::*;

    categories (id) {
        id -> Int8,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        slug -> Varchar,
        #[max_length = 7]
        color -> Nullable<Varchar>,
        description -> Nullable<Text>,
        parent_id -> Nullable<Int8>,
        sort_order -> Nullable<Int4>,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::pg::sql_types::*;

    note_tags (note_id, tag_id) {
        note_id -> Int8,
        tag_id -> Int8,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::pg::sql_types::*;

    reading_notes (id) {
        id -> Int8,
        book_id -> Int8,
        #[max_length = 200]
        title -> Nullable<Varchar>,
        content -> Text,
        #[max_length = 20]
        note_type -> Nullable<Varchar>,
        #[max_length = 50]
        page_reference -> Nullable<Varchar>,
        is_favorite -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::pg::sql_types::*;

    reading_status (id) {
        id -> Int8,
        book_id -> Int8,
        #[max_length = 20]
        status -> Varchar,
        rating -> Nullable<Int4>,
        start_date -> Nullable<Date>,
        finish_date -> Nullable<Date>,
        current_page -> Nullable<Int4>,
        reading_progress -> Nullable<Numeric>,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::pg::sql_types::*;

    tags (id) {
        id -> Int8,
        #[max_length = 30]
        name -> Varchar,
        #[max_length = 30]
        slug -> Varchar,
        usage_count -> Nullable<Int4>,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(book_categories -> books (book_id));
diesel::joinable!(book_categories -> categories (category_id));
diesel::joinable!(book_tags -> books (book_id));
diesel::joinable!(book_tags -> tags (tag_id));
diesel::joinable!(note_tags -> reading_notes (note_id));
diesel::joinable!(note_tags -> tags (tag_id));
diesel::joinable!(reading_notes -> books (book_id));
diesel::joinable!(reading_status -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    book_categories,
    book_tags,
    books,
    categories,
    note_tags,
    reading_notes,
    reading_status,
    tags,
);
