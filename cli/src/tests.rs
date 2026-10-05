use clap::Parser;
use serde_json::{json, Value};

use super::{build_snapshot_config, config::CodeSnapCLIConfig, CLI};
use codesnap::config::SnapshotConfig;

fn custom_config() -> Value {
    let mut config: Value = serde_json::from_str(include_str!("../config.json")).unwrap();
    let snapshot = &mut config["snapshot_config"];
    snapshot["line_number_color"] = json!("#8c8fa1");
    snapshot["scale_factor"] = json!(2);
    snapshot["title"] = json!("Configured title");
    snapshot["window"]["border"] = json!({"width": 2.5, "color": "#123456"});
    snapshot["code_config"]["breadcrumbs"]["enable"] = json!(true);
    config
}

fn merge(config: Value, args: &[&str]) -> anyhow::Result<SnapshotConfig> {
    let cli = CLI::try_parse_from(
        [
            "codesnap",
            "--from-code",
            "fn main() {}",
            "--output",
            "test.png",
        ]
        .into_iter()
        .chain(args.iter().copied()),
    )?;
    let config = CodeSnapCLIConfig::from(&config.to_string())?;
    build_snapshot_config(&cli, config.snapshot_config)
}

#[test]
fn omitted_arguments_preserve_config_values() {
    let snapshot = merge(custom_config(), &[]).unwrap();
    assert_eq!(snapshot.line_number_color, "#8c8fa1");
    assert_eq!(snapshot.scale_factor, 2);
    assert_eq!(snapshot.title.as_deref(), Some("Configured title"));
    assert_eq!(snapshot.window.border.width, 2.5);
    assert_eq!(snapshot.window.border.color, "#123456");
    assert!(snapshot.code_config.breadcrumbs.enable);
}

#[test]
fn explicit_arguments_override_config_values() {
    let snapshot = merge(
        custom_config(),
        &[
            "--line-number-color",
            "#abcdef",
            "--scale-factor",
            "4",
            "--title",
            "CLI title",
            "--border-color",
            "#fedcba",
            "--has-border",
            "false",
            "--has-breadcrumbs",
            "false",
        ],
    )
    .unwrap();
    assert_eq!(snapshot.line_number_color, "#abcdef");
    assert_eq!(snapshot.scale_factor, 4);
    assert_eq!(snapshot.title.as_deref(), Some("CLI title"));
    assert_eq!(snapshot.window.border.color, "#fedcba");
    assert_eq!(snapshot.window.border.width, 0.);
    assert!(!snapshot.code_config.breadcrumbs.enable);
}

#[test]
fn explicit_old_defaults_override_config_values() {
    let mut config = custom_config();
    config["snapshot_config"]["window"]["border"]["width"] = json!(0);
    let snapshot = merge(
        config,
        &[
            "--line-number-color",
            "#495162",
            "--scale-factor",
            "3",
            "--border-color",
            "#ffffff30",
            "--has-border=true",
            "--has-breadcrumbs=false",
        ],
    )
    .unwrap();
    assert_eq!(snapshot.line_number_color, "#495162");
    assert_eq!(snapshot.scale_factor, 3);
    assert_eq!(snapshot.window.border.color, "#ffffff30");
    assert_eq!(snapshot.window.border.width, 1.);
    assert!(!snapshot.code_config.breadcrumbs.enable);
}

#[test]
fn border_color_does_not_change_configured_width() {
    for width in [0., 2.5] {
        let mut config = custom_config();
        config["snapshot_config"]["window"]["border"]["width"] = json!(width);
        let snapshot = merge(config, &["--border-color", "#abcdef"]).unwrap();
        assert_eq!(snapshot.window.border.width, width);
        assert_eq!(snapshot.window.border.color, "#abcdef");
    }
}

#[test]
fn enabling_border_preserves_positive_width_or_uses_one() {
    for args in [
        &["--has-border"][..],
        &["--has-border", "true"],
        &["--has-border=true"],
    ] {
        for (width, expected) in [(0., 1.), (2.5, 2.5)] {
            let mut config = custom_config();
            config["snapshot_config"]["window"]["border"]["width"] = json!(width);
            let snapshot = merge(config, args).unwrap();
            assert_eq!(snapshot.window.border.width, expected);
            assert_eq!(snapshot.window.border.color, "#123456");
        }
    }
}

#[test]
fn breadcrumbs_accept_bare_and_explicit_boolean_values() {
    for (args, expected) in [
        (&["--has-breadcrumbs"][..], true),
        (&["--has-breadcrumbs", "true"], true),
        (&["--has-breadcrumbs=true"], true),
        (&["--has-breadcrumbs", "false"], false),
        (&["--has-breadcrumbs=false"], false),
    ] {
        let mut config = custom_config();
        config["snapshot_config"]["code_config"]["breadcrumbs"]["enable"] = json!(!expected);
        let snapshot = merge(config, args).unwrap();
        assert_eq!(snapshot.code_config.breadcrumbs.enable, expected);
        assert_eq!(snapshot.code_config.breadcrumbs.separator, "/");
    }
}

#[test]
fn missing_config_values_use_builtin_defaults() {
    let config = json!({
        "print_eggs": false,
        "snapshot_config": {"background": "#000000"}
    });
    let snapshot = merge(config, &[]).unwrap();
    assert_eq!(snapshot.line_number_color, "#495162");
    assert_eq!(snapshot.scale_factor, 3);
    assert_eq!(snapshot.window.border.width, 1.);
    assert_eq!(snapshot.window.border.color, "#ffffff30");
    assert!(!snapshot.code_config.breadcrumbs.enable);
    assert!(snapshot.title.is_none());
}

#[test]
fn zero_scale_factor_is_rejected_from_cli_and_config() {
    assert!(merge(custom_config(), &["--scale-factor", "0"]).is_err());
    let mut config = custom_config();
    config["snapshot_config"]["scale_factor"] = json!(0);
    assert!(merge(config.clone(), &[]).is_err());
    assert_eq!(
        merge(config, &["--scale-factor", "1"])
            .unwrap()
            .scale_factor,
        1
    );
}
