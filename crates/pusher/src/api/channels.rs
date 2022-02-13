use std::collections::HashMap;
use std::sync::Arc;


use actix_web::{get, web, Responder};



use crate::{HttpResponse};


use serde::{Deserialize, Serialize};
use crate::adapter::Adapter;


#[derive(Debug, Deserialize)]
pub struct AppPath {
    pub app_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct AllQuery {
    pub filter_by_prefix: Option<String>,
    pub info: Option<String>,
}

#[derive(Serialize, Default)]
pub struct Channels {
    pub channels: HashMap<String, ChannelResult>
}

#[derive(Serialize)]
pub struct ChannelResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_count: Option<usize>,
}


#[get("/apps/{app_id}/channels")]
pub async fn all(
    path: web::Path<AppPath>,
    query: web::Query<AllQuery>,
    adapter: web::Data<Arc<dyn Adapter>>,
) -> impl Responder {
    let mut response_payload = Channels::default();

    let ns = adapter.namespace(path.app_id);

    let channels = ns.channels();

    let with_user_count = match &query.info {
        Some(v) => v.contains(&"user_count".to_string()),
        _ => false,
    };

    for channel in channels {
        response_payload.channels.insert(channel.to_string(), ChannelResult {
            user_count: match with_user_count {
                true => Some(ns.users_per_channel(&channel)),
                _ => None,
            },
            subscription_count: None
        });
    }

    HttpResponse::Ok().json(response_payload)
}
