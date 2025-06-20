use std::{collections::HashMap, fs, path::PathBuf, time::Instant};

use package_json_parser::PackageJsonParser;
use rspack_core::{
  rspack_sources::SourceExt, ApplyContext, CompilationFinishModules, CompilationProcessAssets,
  CompilationSeal, CompilerEmit, CompilerOptions, Plugin, PluginContext,
};
use rspack_error::Result;
use rspack_hook::{plugin, plugin_hook};
use up_finder::UpFinder;

#[plugin]
#[derive(Debug)]
pub struct DemoPlugin {
  options: DemoPluginOptions,
}

#[derive(Debug)]
pub struct DemoPluginOptions {
  pub name: String,
}

impl DemoPlugin {
  pub fn new(options: DemoPluginOptions) -> Self {
    Self::new_inner(options)
  }
}

impl Plugin for DemoPlugin {
  fn name(&self) -> &'static str {
    "rspack.DemoPlugin"
  }

  fn apply(
    &self,
    _ctx: PluginContext<&mut ApplyContext>,
    _options: &CompilerOptions,
  ) -> Result<()> {
    // _ctx.context.compiler_hooks

    _ctx
      .context
      .compilation_hooks
      .seal
      .tap(finish_modules::new(self));

    Ok(())
  }

  // fn clear_cache(&self, _id: rspack_core::CompilationId) {
  //   println!("🧹 DemoPlugin 清理缓存: {:?}", _id);
  //   // 触发垃圾回收（如果可用）
  //   #[cfg(feature = "gc")]
  //   if let Some(gc) = js_sys::gc() {
  //     gc();
  //   }
  // }

  // fn clear_cache(&self, _id: CompilationId) {}

  // fn clear_cache(&self, _id: CompilationId) {}
}

#[plugin_hook(CompilationSeal for DemoPlugin)]
async fn finish_modules(&self, compilation: &mut rspack_core::Compilation) -> Result<()> {
  let start_time = Instant::now();
  println!(
    "🎯 DemoPlugin emit 钩子被触发！插件名称: {}",
    self.options.name
  );

  let mut package_json_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

  let module_graph = compilation.get_module_graph();
  for (_module_identifier, module) in module_graph.modules().iter() {
    if module.module_type().is_js_like() {
      let readable_name = module.readable_identifier(&compilation.options.context);
      let path = PathBuf::from(readable_name.to_string());
      let dir = path.parent().unwrap();
      // println!("🟨 JS模块: {}", readable_name);
      let find_up = UpFinder::builder()
        .cwd(dir) // Start from the src directory
        .build();
      let paths = find_up.find_up("package.json");
      if !paths.is_empty() {
        let first = paths.first().unwrap();

        if let Ok(package_json) = PackageJsonParser::parse(first) {
          let name = package_json
            .name
            .map(|n| n.0.to_string())
            .unwrap_or("unknown_name".to_string());
          let version = package_json
            .version
            .map(|v| v.0.to_string())
            .unwrap_or("unknown_version".to_string());

          if let Some(v_map) = package_json_map.get_mut(&name) {
            if let Some(v) = v_map.get_mut(&version) {
              *v += 1;
            } else {
              v_map.insert(version.clone(), 1);
            }
          } else {
            let mut v_map: HashMap<String, i32> = HashMap::new();
            v_map.insert(version.clone(), 1);
            package_json_map.insert(name.clone(), v_map);
          }
        }

        // println!(
        //   "🟨 package.json: {}@{}",
        //   package_json.name.unwrap().0,
        //   package_json.version.unwrap().0
        // );
      }
      // println!("🟨 package.json: {:#?}", paths);
    }
  }

  for (name, v_map) in package_json_map.iter() {
    if v_map.keys().len() > 1 {
      println!("🟨 发现重复的包: {}", name);
      for (version, count) in v_map.iter() {
        println!("{:width$}{:width$}", name, version, width = 30);
      }
      println!("");
    }
  }

  // let stats = compilation.get_stats();
  // let (a, _) = stats.get_assets();

  // 显式提示完成
  println!("demo plugin processAssets completed");
  println!("🟨 耗时: {:?}", start_time.elapsed());
  Ok(())
}
