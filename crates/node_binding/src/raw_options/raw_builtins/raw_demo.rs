use napi_derive::napi;

#[derive(Debug)]
#[napi(object, object_to_js = false)]
pub struct RawDemoPluginOptions {
  pub name: String,
}

impl From<RawDemoPluginOptions> for rspack_plugin_demo::DemoPluginOptions {
  fn from(options: RawDemoPluginOptions) -> Self {
    Self { name: options.name }
  }
}
