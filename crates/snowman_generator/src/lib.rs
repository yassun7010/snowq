mod error;
mod model;
mod sql;
mod traits;

pub use error::Error;

use itertools::Itertools;
pub use model::insert_typeddict::{
    generate_insert_typeddict, generate_insert_typeddicts, get_insert_typeddict_modules,
    InsertTypedDictOptions,
};
pub use model::pydantic::{
    generate_pydantic_model, generate_pydantic_models, get_pydantic_modules, PydanticOptions,
};
pub use model::update_typeddict::{
    generate_update_typeddict, generate_update_typeddicts, get_update_typeddict_modules,
    UpdateTypedDictOptions,
};
use snowman_connector::query::DatabaseSchema;
use snowman_connector::schema::Table;
pub use sql::generate_sql_definition;
pub use traits::ToPython;

pub fn generate_modlue_init_py(database_names: &[&str]) -> String {
    database_names
        .iter()
        .map(|database_name| format!("from .import {database_name} as {database_name}"))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn generate_import_modules(module_names: &[&str]) -> String {
    module_names
        .iter()
        .map(|module_name| format!("import {}", module_name))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}

pub fn generate_module_docs() -> &'static str {
    r#"#
# Code generated by snowman; DO NOT EDIT.
#
# For more information about snowman,
# please refer to https://github.com/yassun7010/snowman-py .
#
"#
}

pub fn generate_type_checking(inner_code: &str) -> String {
    "if typing.TYPE_CHECKING:\n".to_string()
        + &inner_code
            .split('\n')
            .map(|line| {
                if line.is_empty() {
                    line.to_string()
                } else {
                    "    ".to_string() + line
                }
            })
            .join("\n")
}

pub async fn generate_schema_python_code(
    tables: &[Table],
    pydantic_options: &PydanticOptions,
    insert_typeddict_options: &InsertTypedDictOptions,
    update_typeddict_options: &UpdateTypedDictOptions,
) -> Result<String, crate::Error> {
    let src = if tables.is_empty() {
        generate_module_docs().to_string()
    } else {
        itertools::join(
            [
                generate_module_docs(),
                &generate_import_modules(
                    &itertools::chain!(
                        get_insert_typeddict_modules(),
                        get_update_typeddict_modules(),
                        get_pydantic_modules(),
                    )
                    .unique()
                    .collect::<Vec<&str>>(),
                ),
                &generate_type_checking(&itertools::join(
                    [
                        &generate_insert_typeddicts(tables, insert_typeddict_options),
                        &generate_update_typeddicts(tables, update_typeddict_options),
                    ],
                    "\n",
                )),
                &generate_pydantic_models(
                    tables,
                    pydantic_options,
                    insert_typeddict_options,
                    update_typeddict_options,
                ),
            ],
            "\n",
        )
    };

    Ok(src)
}

pub async fn generate_database_init_python_code(
    schemas: &[&DatabaseSchema],
) -> Result<String, crate::Error> {
    let schema_names = schemas
        .iter()
        .map(|schema| schema.schema_module())
        .collect::<Vec<_>>();

    let src = itertools::join(
        [
            generate_module_docs(),
            &generate_modlue_init_py(
                &schema_names
                    .iter()
                    .map(AsRef::as_ref)
                    .collect::<Vec<&str>>(),
            ),
        ],
        "\n",
    );

    Ok(src)
}
