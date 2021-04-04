use neon::prelude::*;
use sniper_core::sniper::*;
use neon::register_module;

/*pub fn sniper(mut cx: FunctionContext) -> JsResult<JsObject> {
    Ok(cx.string("hello node"))
    let sniperObject=sniper_core::sniper::Sniper::new()
}*/
struct SniperNode {
    sniper: Sniper,
}

impl Finalize for SniperNode {}

fn start_sniper(mut cx: FunctionContext) -> JsResult<JsBox<SniperNode>> {
    let config_path = cx.argument::<JsString>(0)?.value(&mut cx);
    //let language = cx.argument::<JsString>(1)?.value(&mut cx);
    //let mut this = sniper.Sniper::new(&config_path.value());
    //this.set_language(&language.value());
    Ok(cx.boxed(SniperNode { sniper:Sniper::new(&config_path) }))
}

register_module!(mut m, {
    //m.export_class("sniperNode",sniperNode)?;
    m.export_function("startSniper",start_sniper)?;
    Ok(())
});

