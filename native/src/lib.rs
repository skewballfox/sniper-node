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
            //let language: Handle<JsString> = cx.argument::<JsString>(1)?;
            //this.set_language(&language.value());
            Ok(Sniper::new(&config_path.value()))
        }
        //TODO:Define some sort of behavior
        /*method get(mut cx) {
            let attr: String = cx.argument::<JsString>(0)?.value();

            let this = cx.this();

            match &attr[..]{
                "language" => {
                    let language = {
                        let guard = cx.lock();
                        let sniper = this.borrow(&guard);
                        sniper.language
                    };
                    Ok(cx.string(&language).upcast())
                },
                _ => cx.throw_type_error("Property does not exist")
            }
        }*/
    }
}
register_module!(mut m, {
    m.export_class::<JsSniper>("Sniper")?;
    Ok(())
});
