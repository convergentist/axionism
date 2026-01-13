#[cfg(test)]

mod tests {

    use atelier_data::config::Config;
    use std::{env, path::Path};

    #[test]
    fn load_from_toml() {
        // --- Working directory
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let workspace_root = Path::new(manifest_dir).join("test").join("configs");

        // --- Template file (toml)
        let template_file = workspace_root.join("template.toml");

        println!("template_file: {:?}", template_file);

        let test_config = Config::from_toml(template_file.to_str().unwrap())
            .unwrap()
            .clone();

        println!("test_config {:?}", test_config);
    }
}
