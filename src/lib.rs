mod node_client;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use dashmap::{DashMap, DashSet};
use neon::event::EventQueue;
use neon::prelude::*;
use neon::register_module;
use node_client::SniperNodeClient;
use once_cell::sync::Lazy;
use qp_trie::Trie;
use sniper_common::{
    client::{init_client, tarpc_context, tokio},
    service::SniperServiceClient,
};

pub struct Target {
    uri: String,
    language: String,

    triggers: Trie<Vec<u8>, String>,
}
impl Target {}
//static INSTANCE: OnceCell<SniperNode>=OnceCell::new();
/*
impl SniperNode {
    fn run()


}
*/

static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

pub fn init(mut cx: FunctionContext) -> JsResult<JsBox<Arc<SniperNodeClient>>> {
    println!("starting initialization");
    let session_id = cx.argument::<JsString>(0).unwrap().value(&mut cx).clone();
    let queue = EventQueue::new(&mut cx);

    println!("connecting to server");

    Ok(cx.boxed(SniperNodeClient::new(session_id, queue)))
}

//TODO: needs some kind of target blacklist for situation
//where target isn't viable
//TODO: find a way to cache client connection that doesn't lead to connection reset

//TODO: find a way to cache queue and RT that doesn't lead to runtime hang
//this link may be helpful: https://github.com/neon-bindings/neon/issues/560

//let config_path = cx.argument::<JsString>(0)?.value(&mut cx);
//Ok(cx.boxed(SniperNode { sniper:Sniper::new(&config_path) }))

fn add_target(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let sniper_client = cx.argument::<JsBox<SniperNodeClient>>(0)?;
    //let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);

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
            .add_target(
                tarpc_context(),
                sniper_client.session_id.clone(),
                uri,
                language,
            )
            .await
            .unwrap();
    });

    println!("target added");
    Ok(cx.undefined())
}

fn get_triggers(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let sniper_client = cx.argument::<JsBox<Arc<SniperNodeClient>>>(0)?;
    let callback = cx
        .argument::<JsFunction>(3)?
        // Root the function so it can moved to the async block
        .root(&mut cx);
    let Target = cx.argument::<JsBox<Target>>(0)?;
    //let queue=get_queue().lock().unwrap();

    let session_id = cx.argument::<JsString>(0)?.value(&mut cx).clone();
    let uri = cx.argument::<JsString>(1)?.value(&mut cx).clone();
    //let rt=get_rt();
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        println!("trigger block started");
        //let client=get_client().lock().unwrap();
        let client = init_client().await;
        println!("got client");
        let triggers = client
            .get_triggers(tarpc_context(), session_id, uri)
            .await
            .unwrap();
        println!("{:?}", triggers);

        //let queue=get_queue().lock().unwrap();
        sniper_client.queue.send(move |mut cx| {
            // "Un-root" the callback
            let callback = callback.into_inner(&mut cx);

            let jstrigs = JsArray::new(&mut cx, triggers.len() as u32);
            for (i, obj) in triggers.iter().enumerate() {
                let value = JsString::new(&mut cx, obj);
                jstrigs.set(&mut cx, i as u32, value).unwrap();
            }

            // Pieces of data required to invoke the callback
            let this = cx.undefined();
            let args = vec![
                // This is a Node style callback where the first argument is the error
                // Even though this code is infallible, using this format allows us
                // more easily promisify from JavaScript
                cx.null().upcast::<JsValue>(),
                jstrigs.upcast::<JsValue>(),
            ];

            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    Ok(cx.undefined())
}

fn get_snippet(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx
        .argument::<JsFunction>(2)?
        // Root the function so it can moved to the async block
        .root(&mut cx);
    let queue = cx.queue();
    //let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let language = cx.argument::<JsString>(0)?.value(&mut cx);
    let snippet_key = cx.argument::<JsString>(1)?.value(&mut cx);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let client = init_client().await;
        let snippet = client
            .get_snippet(tarpc_context(), language, snippet_key)
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
    cx.export_function("init", init)?;
    cx.export_function("add_target", add_target)?;
    cx.export_function("get_triggers", get_triggers)?;
    cx.export_function("get_snippet", get_snippet)?;
    Ok(())
}
