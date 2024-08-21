#[cfg(all(feature = "js", not(target_family = "wasm")))]
pub mod js;

#[cfg(all(feature = "js", not(target_family = "wasm")))]
pub mod js_v1;

#[cfg(all(feature = "js", not(target_family = "wasm")))]
use {
    std::{
        sync::Arc,
        rc::Rc,
    },
    crate::loader::DecisionLoader,
    self::js::{
        function::*,
        module::{
            console::ConsoleListener,
            zen::ZenListener,
        },
    },
    super::custom_node_adapter::CustomNodeAdapter,
};

#[derive(Default, Clone)]
pub struct FunctionRuntime {
    #[cfg(all(feature = "js", not(target_family = "wasm")))]
    js_function: Option<Rc<Function>>,
}

impl FunctionRuntime {
    #[cfg(all(feature = "js", not(target_family = "wasm")))]
    pub async fn get_or_insert_js_function<L: DecisionLoader + 'static, A: CustomNodeAdapter + 'static>(&mut self, loader: Arc<L>, adapter: Arc<A>) -> anyhow::Result<Rc<Function>> {
        if let Some(function) = &self.js_function {
            return Ok(function.clone());
        }

        let function = Function::create(FunctionConfig {
            listeners: Some(vec![
                Box::new(ConsoleListener),
                Box::new(ZenListener {
                    loader,
                    adapter,
                }),
            ]),
        })
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
        let rc_function = Rc::new(function);
        self.js_function.replace(rc_function.clone());

        Ok(rc_function)
    }
}
