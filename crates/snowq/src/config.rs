use snowq_config::{Config, ConfigV1};
use snowq_generator::PydanticOptions;

pub fn get_pydantic_options(config: &Config) -> PydanticOptions {
    match config {
        Config::V1(ConfigV1 {
            pydantic: pydantic_options,
            ..
        }) => PydanticOptions {
            model_name_prefix: pydantic_options.model_name_prefix.clone(),
            model_name_suffix: pydantic_options.model_name_suffix.clone(),
        },
    }
}

pub fn get_python_models_output_dirpath(config: &Config) -> std::path::PathBuf {
    match config {
        Config::V1(ConfigV1 {
            schema: snowq_config::SchemaConfigV1 { output_dir, .. },
            ..
        }) => output_dir.clone(),
    }
}
