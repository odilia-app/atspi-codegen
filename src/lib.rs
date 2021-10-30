macro_rules! include_codegen {
    ($($file: ident),*) => {
        $(
            #[allow(unused_imports, unused_variables, clippy::type_complexity, clippy::bind_instead_of_map, clippy::needless_borrow, clippy::too_many_arguments)]
            pub mod $file {
            include!(concat!(env!("OUT_DIR"), "/", stringify!($file), ".rs"));
            }
            )*
    }
}

include_codegen![
    accessibility,
    accessible,
    action,
    application,
    cache,
    collection,
    component,
    device_event_controller,
    device_event_listener,
    document,
    editable_text,
    event,
    hyperlink,
    hypertext,
    image,
    processed,
    registry,
    selection,
    socket,
    table_cell,
    table,
    text,
    value
];
