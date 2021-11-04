table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    batches (id) {
        id -> Int4,
        cannabis_id -> Int4,
        harvest_date -> Date,
        package_date -> Date,
        final_test_date -> Date,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    cannabis (id) {
        id -> Int4,
        product_id -> Int4,
        family -> Family,
        thc -> Float4,
        cbd -> Float4,
        total_cannabinoids -> Float4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    inventories (id) {
        id -> Int4,
        product_id -> Int4,
        stock -> Int4,
        price -> Float4,
        net_weight -> Float4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    products (id) {
        id -> Int4,
        name -> Varchar,
        category -> Category,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    terpenes (id) {
        id -> Int4,
        cannabis_id -> Int4,
        myrcene -> Float4,
        pinene -> Float4,
        limonene -> Float4,
        caryophyllene -> Float4,
        terpinolene -> Float4,
    }
}

joinable!(batches -> cannabis (cannabis_id));
joinable!(cannabis -> products (product_id));
joinable!(inventories -> products (product_id));
joinable!(terpenes -> cannabis (cannabis_id));

allow_tables_to_appear_in_same_query!(
    batches,
    cannabis,
    inventories,
    products,
    terpenes,
);
