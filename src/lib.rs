use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use neon::event::EventQueue;
use neon::prelude::*;
use neon::register_module;
use once_cell::sync::{Lazy, OnceCell};
use sniper_common::{
    client::{init_client, tarpc_context, tokio},
    service::SniperServiceClient,
};

//static INSTANCE: OnceCell<SniperNode>=OnceCell::new();
/*
impl SniperNode {
    fn run()


}
*/

static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

//TODO: needs some kind of target blacklist for situation
//where target isn't viable
//TODO: find a way to cache client connection that doesn't lead to connection reset

//TODO: find a way to cache queue and RT that doesn't lead to runtime hang
//this link may be helpful: https://github.com/neon-bindings/neon/issues/560

//let config_path = cx.argument::<JsString>(0)?.value(&mut cx);
//Ok(cx.boxed(SniperNode { sniper:Sniper::new(&config_path) }))

fn add_target(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    //let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    //let sniper=cx.argument::<JsBox<SniperNodeClient>>(0).unwrap();
    let session_id = cx.argument::<JsString>(0).unwrap().value(&mut cx).clone();
    let uri = cx.argument::<JsString>(1).unwrap().value(&mut cx).clone();
    let language = cx.argument::<JsString>(2).unwrap().value(&mut cx).clone();
    println!("failed here?");

    println!("nope here");

    //let rt=get_rt.as_ref();
    //let client_lock=sniper.client.clone();
    //let rt=get_rt().as_ref();

    RT.block_on(async move {
        println!("adding target");
        //let client=client_lock.lock().unwrap().clone();//init_client().await;
        let client = init_client().await;
        println!("client: {:?}", client);
        client
            .add_target(tarpc_context(), session_id, uri, language)
            .await
            .unwrap();
    });
    println!("target added");
    Ok(cx.undefined())
}

fn drop_target(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let session_id = cx.argument::<JsString>(0).unwrap().value(&mut cx);
    let uri = cx.argument::<JsString>(1).unwrap().value(&mut cx);
    let language = cx.argument::<JsString>(2).unwrap().value(&mut cx);
    //let handler=global_handler();
    println!("dropping target");

    //let rt=get_rt();
    let rt = tokio::runtime::Runtime::new().unwrap();
    RT.block_on(async move {
        let client = init_client().await;
        //let mut client=get_client().lock().unwrap();
        client
            .drop_target(tarpc_context(), session_id, uri, language)
            .await
            .unwrap();
        //drop(client);
    });
    Ok(cx.undefined())
}

fn get_completions(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx
        .argument::<JsFunction>(3)?
        // Root the function so it can moved to the async block
        .root(&mut cx);
    let queue = cx.queue();
    //let queue=get_queue().lock().unwrap();

    let session_id = cx.argument::<JsString>(0)?.value(&mut cx).clone();
    let uri = cx.argument::<JsString>(1)?.value(&mut cx).clone();
    let input = cx.argument::<JsString>(2)?.value(&mut cx).as_bytes();
    //let rt=get_rt();

    RT.block_on(async move {
        //let client=get_client().lock().unwrap();
        let client = init_client().await;
        println!("got client");
        let completions = client
            .get_completions(tarpc_context(), session_id, uri, input.to_vec())
            .await
            .unwrap();
        println!("{:?}", completions);

        //let queue=get_queue().lock().unwrap();
        queue.send(move |mut cx| {
            // "Un-root" the callback
            let callback = callback.into_inner(&mut cx);

            let jscompletions = JsArray::new(&mut cx, completions.len() as u32);
            for (i, obj) in completions.iter().enumerate() {
                let value = JsString::new(&mut cx, obj);
                jscompletions.set(&mut cx, i as u32, value).unwrap();
            }

            // Pieces of data required to invoke the callback
            let this = cx.undefined();
            let args = vec![
                // This is a Node style callback where the first argument is the error
                // Even though this code is infallible, using this format allows us
                // more easily promisify from JavaScript
                cx.null().upcast::<JsValue>(),
                jscompletions.upcast::<JsValue>(),
            ];

            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    Ok(cx.undefined())
}

fn get_snippet(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx
        .argument::<JsFunction>(3)?
        // Root the function so it can moved to the async block
        .root(&mut cx);
    let queue = cx.queue();
    //let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let session_id = cx.argument::<JsString>(0)?.value(&mut cx).clone();
    let uri = cx.argument::<JsString>(1)?.value(&mut cx).clone();
    let snippet_key = cx.argument::<JsString>(2)?.value(&mut cx);

    RT.block_on(async move {
        let client = init_client().await;
        let snippet = client
            .get_snippet(tarpc_context(), session_id, uri, snippet_key)
            .await
            .unwrap()
            .unwrap();
        println!("Snippet: {:?}", snippet);
        //let queue=get_queue().lock().unwrap();
        queue.send(move |mut cx| {
            // "Un-root" the callback
            let callback = callback.into_inner(&mut cx);

            let jssnippet = JsArray::new(&mut cx, snippet.len() as u32);
            for (i, obj) in snippet.iter().enumerate() {
                let value = JsString::new(&mut cx, obj);
                jssnippet.set(&mut cx, i as u32, value).unwrap();
            }

            // Pieces of data required to invoke the callback
            let this = cx.undefined();
            let args = vec![
                // This is a Node style callback where the first argument is the error
                // Even though this code is infallible, using this format allows us
                // more easily promisify from JavaScript
                cx.null().upcast::<JsValue>(),
                jssnippet.upcast::<JsValue>(),
            ];

            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    //m.export_class("sniperNode",sniperNode)?;
    //cx.export_function("init", init)?;
    cx.export_function("add_target", add_target)?;
    cx.export_function("drop_target", drop_target)?;
    cx.export_function("get_completions", get_completions)?;
    cx.export_function("get_snippet", get_snippet)?;
    Ok(())
}
