// pub fn get_channel_manager(ch: &Channel) -> Option<Box<dyn ChannelManger>> {
//     match *ch {
//         // Channel::Presence(_) => Some(Box::new(PresenceChannelManager {})),
//         // Channel::Private(_) => Some(Box::new(PrivateChannelManager {})),
//         // Channel::Public(_) => Some(Box::new(PublicChannelManager {})),
//         Channel::Invalid => None
//     }
// }

// pub trait ChannelManger {
//     fn join(
//         &self,
//         adapter: &dyn Adapter,
//         id: ,
//         namespace: Namespace,
//         message: &PusherSubscribeMessage,
//     ) -> Result<(), PusherError> {
//         let app_id = ws.app.as_ref().unwrap().id.clone();
//
//         adapter
//             .namespace(&app_id)
//             .add_to_channel(id, message.channel.to_string());
//
//         Ok(())
//     }
//
//     fn leave(
//         &self,
//         adapter: &dyn Adapter,
//         id: Socket,
//         namespace: Namespace,
//         message: &PusherUnsubscribeMessage,
//     ) -> Result<(), PusherError> {
//         let mut namespace = adapter.namespace(&ws.app.unwrap().id);
//
//         namespace.remove_from_channel(ws.id, message.channel.to_string());
//
//         Ok(())
//     }
// }
//
// pub struct PublicChannelManager {}
//
// impl ChannelManger for PublicChannelManager {
//
// }
//
// pub struct PrivateChannelManager {}
//
// impl ChannelManger for PrivateChannelManager {
//     fn join(
//         &self,
//         adapter: &dyn Adapter,
//         ws: WebSocket,
//         message: &PusherSubscribeMessage,
//     ) -> Result<(), PusherError> {
//         let channel = message.channel.to_string();
//
//         let ap = AuthPayload::new(
//             message.auth.clone().unwrap().clone(),
//             ws.id.to_string(),
//             channel.clone(),
//             None,
//         );
//
//         let app = ws.app.as_ref().unwrap();
//
//         match validate_token(
//             &app,
//             &ap,
//         ) {
//             Ok(_) => {
//                 adapter
//                     .namespace(&app.id)
//                     .add_to_channel(Clone::clone(&ws), channel.clone());
//
//                 Ok(())
//             }
//             Err(_) => Err(PusherError::unauth(&message.channel)),
//         }
//     }
// }
//
// pub struct PresenceChannelManager {}
//
// impl ChannelManger for PresenceChannelManager {
//     fn join(
//         &self,
//         adapter: &dyn Adapter,
//         ws: WebSocket,
//         message: &PusherSubscribeMessage,
//     ) -> Result<(), PusherError> {
//         let channel = message.channel.to_string();
//
//         let ap = AuthPayload::new(
//             message.auth.as_ref().unwrap().clone(),
//             ws.id.to_string(),
//             channel.clone(),
//             message.channel_data.clone(),
//         );
//
//         let app = ws.app.as_ref().unwrap().clone();
//
//         match validate_token(
//             &app,
//             &ap,
//         ) {
//             Ok(_) => {
//                 let app_id = ws.app.as_ref().unwrap().id.clone();
//
//                 adapter
//                     .namespace(&app.id)
//                     .add_to_channel(Clone::clone(&ws), channel.clone());
//
//                 Ok(())
//
//             }
//             Err(_) => Err(PusherError::unauth(&message.channel)),
//         }
//     }
// }
