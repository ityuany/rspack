use std::{
  collections::HashMap,
  convert::TryFrom,
  path::{Path, PathBuf},
  str::FromStr,
  sync::Arc,
};

use rspack_core::{BoxLoader, BoxPlugin, CompilerOptions, ModuleType, PluginExt, TargetPlatform};
use rspack_plugin_css::pxtorem::options::PxToRemOptions;
use rspack_plugin_html::config::HtmlPluginConfig;
use rspack_regex::RspackRegex;
use schemars::JsonSchema;
use serde::Deserialize;

macro_rules! impl_serde_default {
  ($name:ident) => {
    impl Default for $name {
      fn default() -> Self {
        serde_json::from_str("{}").expect("Failed to parse default config")
      }
    }
  };
}

fn default_entry() -> HashMap<String, EntryItem> {
  let mut map = HashMap::new();
  map.insert(
    "main".to_string(),
    EntryItem {
      import: vec!["./index".to_string()],
      runtime: None,
    },
  );
  map
}

fn default_public_path() -> String {
  "auto".to_string()
}

fn default_target() -> Vec<String> {
  vec!["web".to_string(), "es2022".to_string()]
}
fn enable_runtime_by_default() -> Option<String> {
  Some("runtime".to_string())
}

fn default_chunk_filename() -> String {
  "[name][ext]".to_string()
}

fn default_optimization_module_ids() -> String {
  "named".to_string()
}

fn default_optimization_side_effects() -> String {
  "false".to_string()
}

fn true_by_default() -> bool {
  true
}

/// The configuration is used to configure the test in Rust.
/// The structure should be closed to the webpack configuration.
#[derive(Debug, JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TestConfig {
  #[serde(default = "default_entry")]
  pub entry: HashMap<String, EntryItem>,
  #[serde(default)]
  pub builtins: Builtins,
  #[serde(default = "default_target")]
  pub target: Vec<String>,
  #[serde(default)]
  pub output: Output,
  #[serde(default)]
  pub module: Module,
  #[serde(default)]
  pub optimization: Optimization,
  #[serde(default)]
  pub devtool: String,
}

#[derive(Debug, JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Optimization {
  // True by default to reduce code in snapshots.
  #[serde(default = "true_by_default")]
  pub remove_available_modules: bool,
  #[serde(default = "default_optimization_module_ids")]
  pub module_ids: String,
  #[serde(default = "default_optimization_side_effects")]
  pub side_effects: String,
}

#[derive(Debug, JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntryItem {
  pub import: Vec<String>,
  #[serde(default = "enable_runtime_by_default")]
  pub runtime: Option<String>,
}

#[derive(Debug, Default, JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Minification {
  #[serde(default)]
  pub passes: usize,
  #[serde(default)]
  pub drop_console: bool,
  #[serde(default)]
  pub pure_funcs: Vec<String>,
}

#[derive(Debug, JsonSchema, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresetEnv {
  targets: Vec<String>,
  mode: Option<String>,
  core_js: Option<String>,
}

#[derive(Debug, JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Builtins {
  #[serde(default)]
  pub define: HashMap<String, String>,
  #[serde(default)]
  pub postcss: Postcss,
  #[serde(default)]
  pub html: Vec<HtmlPluginConfig>,
  #[serde(default)]
  pub minify_options: Option<Minification>,
  #[serde(default)]
  pub tree_shaking: bool,
  #[serde(default)]
  pub preset_env: Option<PresetEnv>,
  #[serde(default)]
  pub css: Css,
}

#[derive(Debug, JsonSchema, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Css {
  #[serde(default)]
  pub modules: ModulesConfig,
}

#[derive(Debug, JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct ModulesConfig {
  pub locals_convention: String,
  pub local_ident_name: String,
  pub exports_only: bool,
}

impl Default for ModulesConfig {
  fn default() -> Self {
    Self {
      locals_convention: "asIs".to_string(),
      local_ident_name: "[path][name][ext]__[local]".to_string(),
      exports_only: false,
    }
  }
}

#[derive(Debug, JsonSchema, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Postcss {
  #[serde(default)]
  pub pxtorem: Option<PxToRem>,
}

#[derive(Debug, JsonSchema, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct PxToRem {
  pub root_value: Option<u32>,
  pub unit_precision: Option<u32>,
  pub selector_black_list: Option<Vec<String>>,
  pub prop_list: Option<Vec<String>>,
  pub replace: Option<bool>,
  pub media_query: Option<bool>,
  pub min_pixel_value: Option<f64>,
}

impl From<PxToRem> for PxToRemOptions {
  fn from(value: PxToRem) -> Self {
    Self {
      root_value: value.root_value,
      unit_precision: value.unit_precision,
      selector_black_list: value.selector_black_list,
      prop_list: value.prop_list,
      replace: value.replace,
      media_query: value.media_query,
      min_pixel_value: value.min_pixel_value,
    }
  }
}

#[derive(Debug, JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Output {
  #[serde(default = "default_public_path")]
  pub public_path: String,
  #[serde(default = "default_chunk_filename")]
  pub filename: String,
  #[serde(default = "default_chunk_filename")]
  pub chunk_filename: String,
  #[serde(default = "default_chunk_filename")]
  pub css_filename: String,
  #[serde(default = "default_chunk_filename")]
  pub css_chunk_filename: String,
}

#[derive(Debug, JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Module {
  #[serde(default)]
  pub rules: Vec<Rule>,
}

#[derive(Debug, JsonSchema, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct Rule {
  pub test: Option<ModuleRuleTest>,
  pub r#use: Vec<ModuleRuleUse>,
  pub side_effect: Option<bool>,
  pub r#type: Option<String>,
}

#[derive(Debug, JsonSchema, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum ModuleRuleTest {
  Regexp { matcher: String },
}

#[derive(Debug, JsonSchema, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub struct ModuleRuleUse {
  builtin_loader: String,
  options: Option<String>,
}

impl_serde_default!(TestConfig);
impl_serde_default!(Output);
impl_serde_default!(Builtins);
impl_serde_default!(EntryItem);
impl_serde_default!(Module);
impl_serde_default!(Optimization);

impl TestConfig {
  pub fn apply(self, context: PathBuf) -> (CompilerOptions, Vec<BoxPlugin>) {
    use rspack_core as c;

    impl From<PresetEnv> for c::PresetEnv {
      fn from(preset_env: PresetEnv) -> Self {
        Self {
          mode: preset_env
            .mode
            .and_then(|mode| c::string_to_mode(mode.as_str())),
          targets: preset_env.targets,
          core_js: preset_env.core_js,
        }
      }
    }

    let targets = self
      .builtins
      .preset_env
      .as_ref()
      .map(|preset_env| preset_env.targets.clone())
      .unwrap_or_default();

    assert!(context.is_absolute());
    let options = CompilerOptions {
      context: c::Context::new(context.clone()),
      entry: self
        .entry
        .into_iter()
        .map(|(k, v)| {
          (
            k,
            c::EntryItem {
              import: v.import,
              runtime: v
                .runtime
                .map(Some)
                // Splitting runtime code into a separate chunk
                // is friendly to snapshot testing.
                .unwrap_or_else(|| Some("runtime".to_string())),
            },
          )
        })
        .collect(),
      output: c::OutputOptions {
        filename: c::Filename::from_str(&self.output.filename).expect("Should exist"),
        chunk_filename: c::Filename::from_str(&self.output.chunk_filename).expect("Should exist"),
        css_filename: c::Filename::from_str(&self.output.css_filename).expect("Should exist"),
        css_chunk_filename: c::Filename::from_str(&self.output.css_chunk_filename)
          .expect("Should exist"),
        asset_module_filename: c::Filename::from_str("[hash][ext][query]").expect("Should exist"),
        public_path: c::PublicPath::String("/".to_string()),
        unique_name: "__rspack_test__".to_string(),
        path: context.join("dist"),
        library: None,
        strict_module_error_handling: false,
      },
      mode: c::Mode::None,
      target: c::Target::new(&self.target).expect("Can't construct target"),
      resolve: c::Resolve {
        extensions: Some(
          [".js", ".jsx", ".ts", ".tsx", ".json", ".d.ts", ".css"]
            .into_iter()
            .map(|i| i.to_string())
            .collect(),
        ),
        ..Default::default()
      },
      builtins: c::Builtins {
        define: self.builtins.define,
        tree_shaking: self.builtins.tree_shaking,
        minify_options: self.builtins.minify_options.map(|op| c::Minification {
          passes: op.passes,
          drop_console: op.drop_console,
          pure_funcs: op.pure_funcs,
        }),
        preset_env: self.builtins.preset_env.map(Into::into),
        ..Default::default()
      },
      module: c::ModuleOptions {
        rules: self
          .module
          .rules
          .into_iter()
          .map(|rule| c::ModuleRule {
            test: rule.test.map(|test| match test {
              ModuleRuleTest::Regexp { matcher } => c::ModuleRuleCondition::Regexp(
                RspackRegex::new(&matcher).expect("should be valid regex"),
              ),
            }),
            r#use: rule
              .r#use
              .into_iter()
              .map(|i| match i.builtin_loader.as_str() {
                "builtin:sass-loader" => Arc::new(rspack_loader_sass::SassLoader::new(
                  i.options
                    .map(|options| {
                      serde_json::from_str::<rspack_loader_sass::SassLoaderOptions>(&options)
                        .expect("should give a right loader options")
                    })
                    .unwrap_or_default(),
                )) as BoxLoader,
                _ => panic!("should give a right loader"),
              })
              .collect::<Vec<BoxLoader>>(),
            side_effects: rule.side_effect,
            r#type: rule
              .r#type
              .map(|i| ModuleType::try_from(i.as_str()).expect("should give a right module_type")),
            ..Default::default()
          })
          .collect(),
        ..Default::default()
      },
      devtool: c::Devtool::from(self.devtool),
      externals: Default::default(),
      externals_type: c::ExternalType::Auto,
      stats: Default::default(),
      snapshot: Default::default(),
      cache: c::CacheOptions::Disabled,
      experiments: Default::default(),
      dev_server: Default::default(),
      node: c::NodeOption {
        dirname: "mock".to_string(),
      },
      optimization: c::Optimization {
        remove_available_modules: self.optimization.remove_available_modules,
        side_effects: c::SideEffectOption::from(self.optimization.side_effects.as_str()),
      },
    };
    let mut plugins = Vec::new();
    for html in self.builtins.html {
      plugins.push(rspack_plugin_html::HtmlPlugin::new(html).boxed());
    }
    plugins.push(
      rspack_plugin_css::CssPlugin::new(rspack_plugin_css::plugin::CssConfig {
        targets,
        postcss: rspack_plugin_css::plugin::PostcssConfig {
          pxtorem: self.builtins.postcss.pxtorem.map(|i| i.into()),
        },
        modules: rspack_plugin_css::plugin::ModulesConfig {
          locals_convention: rspack_plugin_css::plugin::LocalsConvention::from_str(
            &self.builtins.css.modules.locals_convention,
          )
          .expect("Invalid css.modules.locals_convention"),
          local_ident_name: rspack_plugin_css::plugin::LocalIdentName::from(
            self.builtins.css.modules.local_ident_name,
          ),
          exports_only: self.builtins.css.modules.exports_only,
        },
      })
      .boxed(),
    );
    plugins.push(
      rspack_plugin_asset::AssetPlugin::new(rspack_plugin_asset::AssetConfig {
        parse_options: options.module.parser.as_ref().and_then(|x| x.asset.clone()),
      })
      .boxed(),
    );
    plugins.push(rspack_plugin_json::JsonPlugin {}.boxed());
    match &options.target.platform {
      TargetPlatform::Web => {
        plugins.push(rspack_plugin_runtime::ArrayPushCallbackChunkFormatPlugin {}.boxed());
        plugins.push(rspack_plugin_runtime::RuntimePlugin {}.boxed());
        plugins.push(rspack_plugin_runtime::CssModulesPlugin {}.boxed());
        plugins.push(rspack_plugin_runtime::JsonpChunkLoadingPlugin {}.boxed());
      }
      TargetPlatform::Node(_) => {
        plugins.push(rspack_plugin_runtime::CommonJsChunkFormatPlugin {}.boxed());
        plugins.push(rspack_plugin_runtime::RuntimePlugin {}.boxed());
        plugins.push(rspack_plugin_runtime::CommonJsChunkLoadingPlugin {}.boxed());
      }
      _ => {
        plugins.push(rspack_plugin_runtime::RuntimePlugin {}.boxed());
      }
    };
    if options.dev_server.hot {
      plugins.push(rspack_plugin_runtime::HotModuleReplacementPlugin {}.boxed());
    }
    plugins.push(rspack_plugin_runtime::BasicRuntimeRequirementPlugin {}.boxed());
    if options.experiments.lazy_compilation {
      plugins.push(rspack_plugin_runtime::LazyCompilationPlugin {}.boxed());
    }
    plugins.push(rspack_plugin_externals::ExternalPlugin::default().boxed());
    plugins.push(rspack_plugin_javascript::JsPlugin::new().boxed());
    plugins.push(
      rspack_plugin_devtool::DevtoolPlugin::new(rspack_plugin_devtool::DevtoolPluginOptions {
        inline: options.devtool.inline(),
        append: !options.devtool.hidden(),
        namespace: options.output.unique_name.clone(),
        columns: !options.devtool.cheap(),
        no_sources: options.devtool.no_sources(),
        public_path: None,
      })
      .boxed(),
    );
    if self.optimization.module_ids == "named" {
      plugins.push(rspack_ids::NamedModuleIdsPlugin::default().boxed());
    } else {
      plugins.push(rspack_ids::DeterministicModuleIdsPlugin::default().boxed());
    }
    plugins.push(rspack_ids::StableNamedChunkIdsPlugin::new(None, None).boxed());
    // Notice the plugin need to be placed after SplitChunksPlugin
    plugins.push(rspack_plugin_remove_empty_chunks::RemoveEmptyChunksPlugin.boxed());

    (options, plugins)
  }

  pub fn from_fixture(fixture_path: &Path) -> Self {
    let config_path = fixture_path.join("test.config.json");
    // let config = if config_path
    let test_config: TestConfig = if config_path.exists() {
      let config_content =
        std::fs::read_to_string(config_path).expect("test.config.json should exist");
      serde_json::from_str(&config_content).expect("should be valid json")
    } else {
      serde_json::from_str("{}").expect("should be valid json")
    };
    test_config
  }
}

pub fn apply_from_fixture(fixture_path: &Path) -> (CompilerOptions, Vec<BoxPlugin>) {
  let test_config = TestConfig::from_fixture(fixture_path);
  test_config.apply(fixture_path.to_path_buf())
}

pub fn add_entry_runtime(mut options: CompilerOptions) -> CompilerOptions {
  for (_, entry) in options.entry.iter_mut() {
    entry.runtime = Some("runtime".to_string());
  }
  options
}