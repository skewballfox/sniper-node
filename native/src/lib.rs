use neon::declare_types;
use neon::prelude::*;
use sniper_core::sniper::*;

/*pub fn sniper(mut cx: FunctionContext) -> JsResult<JsObject> {
    Ok(cx.string("hello node"))
    let sniperObject=sniper_core::sniper::Sniper::new()
}*/
declare_types! {
    pub class JsSniper for Sniper {
        init(mut cx) {
            let config_path: Handle<JsString> = cx.argument::<JsString>(0)?;
            let language: Handle<JsString> = cx.argument::<JsString>(1)?;
            let mut this=Sniper::new(&config_path.value());
            this.set_language(&language.value());
            Ok(this)
        }
        //TODO:Define some sort of behavior
        /*method set_target(mut cx) {
            let this=cx.this()
        }*/
    }
}
register_module!(mut m, {
    m.export_class::<JsSniper>("Sniper")?;
    Ok(())
});
