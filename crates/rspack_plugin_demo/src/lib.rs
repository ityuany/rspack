use rspack_core::{
  rspack_sources::SourceExt, ApplyContext, CompilationProcessAssets, CompilerEmit, CompilerOptions,
  Plugin, PluginContext,
};
use rspack_error::Result;
use rspack_hook::{plugin, plugin_hook};

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
      .process_assets
      .tap(process_assets::new(self));

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
#[plugin_hook(CompilationProcessAssets for DemoPlugin)]
async fn process_assets(&self, compilation: &mut rspack_core::Compilation) -> Result<()> {
  println!(
    "🎯 DemoPlugin emit 钩子被触发！插件名称: {}",
    self.options.name
  );

  // 优化：减少asset操作的复杂度
  let mut assets_to_modify = Vec::new();

  let mut dep_str = String::new();

  let module_graph = compilation.get_module_graph();
  for (_module_identifier, module) in module_graph.modules().iter() {
    if module.module_type().is_js_like() {
      let readable_name = module.readable_identifier(&compilation.options.context);
      println!("🟨 JS模块: {}", readable_name);
      dep_str.push_str(&format!("{}\n", readable_name));
    }
  }

  for (filename, _) in compilation.assets().iter() {
    if filename.ends_with(".js") {
      println!("🎯 修改的文件: {}", filename);
      assets_to_modify.push(filename.clone());
      break; // 只修改第一个
    }
  }

  let stats = compilation.get_stats();
  let (a, _) = stats.get_assets();

  let mut str = String::new();

  for f in a.iter() {
    str.push_str(&format!("{}:{}kb\n", f.name, f.size / 1024.0));
  }

  let demo_message = format!(
    r#"
    /* DemoPlugin 生成的注释 - 插件名称: {} 
    dep_str:{}
    {}
    */"#,
    self.options.name, dep_str, str
  );

  for filename in assets_to_modify {
    compilation.update_asset(&filename, |source, info| {
      let current_content = source.source().to_string();
      let new_content = format!("{}{}{}", demo_message, current_content, "\nconst b = 12;");
      // let new_content = format!("{}{}", demo_message, current_content);
      let new_source = rspack_core::rspack_sources::RawSource::from(new_content).boxed();
      Ok((new_source, info))
    })?;
  }

  // 显式提示完成
  println!("demo plugin processAssets completed");
  Ok(())
}
