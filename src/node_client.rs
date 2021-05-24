use dashmap::DashMap;
use neon::event::EventQueue;
use neon::prelude::*;
use once_cell::sync::Lazy;
use sniper_common::{
    client::{init_client, tarpc_context, tokio},
    service::SniperServiceClient,
};
use std::sync::Arc;

use crate::{Target, RT};

pub struct SniperNodeClient {
    pub session_id: String,
    pub(crate) queue: EventQueue,
    pub(crate) targets: dashmap::DashMap<String, Target>,
}
impl Finalize for SniperNodeClient {
    fn finalize<'a, C: Context<'a>>(self, cx: &mut C) {
        println!("dropping target");
        let global = cx.global();
        let emit = global
            .get(cx, "emit")
            .unwrap()
            .downcast::<JsFunction, _>(cx)
            .unwrap();
        RT.block_on(async move {
            let client = init_client().await;
            //let mut client=get_client().lock().unwrap();
            for target in self.targets.iter() {
                client
                    .drop_target(
                        tarpc_context(),
                        self.session_id,
                        target.uri,
                        target.language,
                    )
                    .await
                    .unwrap();
            }
        });
        let args = vec![
            cx.string("gc_point").upcast::<JsValue>(),
            cx.string(self.session_id.clone()).upcast::<JsValue>(),
        ];

        emit.call(cx, global, args).unwrap();
    }
}

impl SniperNodeClient {
    pub fn new(session_id: String, queue: EventQueue) -> Arc<Self> {
        Arc::new(Self {
            session_id,
            queue,
            targets: DashMap::new(),
        })
    }
}
