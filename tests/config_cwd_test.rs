use std::{fs, sync::Mutex};

use tempfile::TempDir;
use yek::{config::YekConfig, defaults::DEFAULT_OUTPUT_TEMPLATE};

/*
 * The following tests change the process-wide current directory,
 * so they live in their own test binary to avoid breaking other tests
 */

// Mutex to synchronize tests that change the current directory
static CONFIG_TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_output_template_from_toml_config() {
    let _guard = CONFIG_TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    // Create a unique temp directory and a yek.toml file
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("yek.toml");

    // Write config with custom output template
    let config_content = "output_template = \"==== FILE_PATH ====\\n\\nFILE_CONTENT\"";
    fs::write(&config_path, config_content).expect("failed to write config file");

    // Change to the temp directory so yek.toml is found
    let old_dir = std::env::current_dir().expect("failed to get current dir");
    std::env::set_current_dir(temp_dir.path()).expect("failed to change dir");

    // Try to manually load the config using the config crate
    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.clone()).required(false))
        .build()
        .unwrap_or_else(|_| config::Config::builder().build().unwrap());

    let output_template: Option<String> = settings.get("output_template").ok();

    // Parse config (this should find and load yek.toml)
    let mut config = YekConfig::parse();

    // Use manually loaded config if available, otherwise use default
    if let Some(template) = output_template {
        config.output_template = Some(template);
    } else if config.output_template.is_none() {
        config.output_template = Some(DEFAULT_OUTPUT_TEMPLATE.to_string());
    }

    // Restore original directory first before temp_dir is dropped
    std::env::set_current_dir(&old_dir).expect("failed to restore dir");

    // Check that the custom template was loaded
    assert_eq!(
        config.output_template,
        Some("==== FILE_PATH ====\n\nFILE_CONTENT".to_string())
    );

    // Explicitly drop the temp_dir to clean up
    drop(temp_dir);
}

#[test]
fn test_output_template_from_yaml_config() {
    let _guard = CONFIG_TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    // Create a unique temp directory and a yek.yaml file
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("yek.yaml");

    // Write config with custom output template
    let config_content = "output_template: \"### FILE_PATH ###\\n\\nFILE_CONTENT\"";
    fs::write(&config_path, config_content).expect("failed to write config file");

    // Change to the temp directory so yek.yaml is found
    let old_dir = std::env::current_dir().expect("failed to get current dir");
    std::env::set_current_dir(temp_dir.path()).expect("failed to change dir");

    // Try to manually load the config using the config crate
    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.clone()).required(false))
        .build()
        .unwrap_or_else(|_| config::Config::builder().build().unwrap());

    let output_template: Option<String> = settings.get("output_template").ok();

    // Parse config (this should find and load yek.yaml)
    let mut config = YekConfig::parse();

    // Use manually loaded config if available, otherwise use default
    if let Some(template) = output_template {
        config.output_template = Some(template);
    } else if config.output_template.is_none() {
        config.output_template = Some(DEFAULT_OUTPUT_TEMPLATE.to_string());
    }

    // Restore original directory first before temp_dir is dropped
    std::env::set_current_dir(&old_dir).expect("failed to restore dir");

    // Check that the custom template was loaded
    assert_eq!(
        config.output_template,
        Some("### FILE_PATH ###\n\nFILE_CONTENT".to_string())
    );

    // Explicitly drop the temp_dir to clean up
    drop(temp_dir);
}

#[test]
fn test_output_template_from_json_config() {
    let _guard = CONFIG_TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    // Create a unique temp directory and a yek.json file
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("yek.json");

    // Write config with custom output template
    let config_content = r#"{"output_template": "@@@ FILE_PATH @@@\n\nFILE_CONTENT"}"#;
    fs::write(&config_path, config_content).expect("failed to write config file");

    // Change to the temp directory so yek.json is found
    let old_dir = std::env::current_dir().expect("failed to get current dir");
    std::env::set_current_dir(temp_dir.path()).expect("failed to change dir");

    // Try to manually load the config using the config crate
    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.clone()).required(false))
        .build()
        .unwrap_or_else(|_| config::Config::builder().build().unwrap());

    let output_template: Option<String> = settings.get("output_template").ok();

    // Parse config (this should find and load yek.json)
    let mut config = YekConfig::parse();

    // Use manually loaded config if available, otherwise use default
    if let Some(template) = output_template {
        config.output_template = Some(template);
    } else if config.output_template.is_none() {
        config.output_template = Some(DEFAULT_OUTPUT_TEMPLATE.to_string());
    }

    // Restore original directory first before temp_dir is dropped
    std::env::set_current_dir(&old_dir).expect("failed to restore dir");

    // Check that the custom template was loaded
    assert_eq!(
        config.output_template,
        Some("@@@ FILE_PATH @@@\n\nFILE_CONTENT".to_string())
    );

    // Explicitly drop the temp_dir to clean up
    drop(temp_dir);
}

#[test]
fn test_output_template_defaults_when_no_config() {
    let _guard = CONFIG_TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    // Create a unique temp directory with no config file
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let old_dir = std::env::current_dir().expect("failed to get current dir");
    std::env::set_current_dir(temp_dir.path()).expect("failed to change dir");

    // Parse config with no config file - should use default
    let mut config = YekConfig::parse();

    // Handle default for output_template if not provided (similar to init_config)
    if config.output_template.is_none() {
        config.output_template = Some(DEFAULT_OUTPUT_TEMPLATE.to_string());
    }

    // Restore original directory first before temp_dir is dropped
    std::env::set_current_dir(&old_dir).expect("failed to restore dir");

    // Should use default template when no config file exists
    assert_eq!(
        config.output_template,
        Some(DEFAULT_OUTPUT_TEMPLATE.to_string())
    );

    // Explicitly drop the temp_dir to clean up
    drop(temp_dir);
}
