use crate::RequestContext;
use generust_example_project_core::{RequestMessage, ResponseMessage, Result};

/// Core application logic, routing [RequestMessage](generust_example_project_core::RequestMessage)s and emitting [ResponseMessage](generust_example_project_core::ResponseMessage)s.
#[derive(Debug)]
pub struct MessageHandler {
  connection_id: uuid::Uuid,
  channel_id: String,
  ctx: RequestContext,
  log: slog::Logger
}

impl MessageHandler {
  pub fn new(connection_id: uuid::Uuid, channel_id: String, ctx: RequestContext) -> MessageHandler {
    let log = ctx
      .log()
      .new(slog::o!("connection" => format!("{}", connection_id), "service" => "message_handler", "channel" => channel_id.clone()));
    MessageHandler {
      connection_id,
      channel_id,
      ctx,
      log
    }
  }

  pub fn connection_id(&self) -> &uuid::Uuid {
    &self.connection_id
  }

  pub fn channel_id(&self) -> &String {
    &self.channel_id
  }

  pub fn ctx(&self) -> &RequestContext {
    &self.ctx
  }

  pub fn on_open(&self) -> Result<Vec<ResponseMessage>> {
    Ok(vec![ResponseMessage::Connected {
      connection_id: *self.connection_id(),
      u: Box::new((*self.ctx.user_profile()).clone()),
      b: !self.ctx.app().verbose()
    }])
  }

  pub fn on_closed(&self) -> Vec<ResponseMessage> {
    Vec::new()
  }

  pub fn on_message(&self, msg: RequestMessage) -> Result<Vec<ResponseMessage>> {
    let mut ret = Vec::new();
    match msg {
      RequestMessage::Ping { v } => ret.push(ResponseMessage::Pong { v }),
      msg => slog::warn!(self.log, "Unhandled RequestMessage [{:?}]", msg)
    }
    Ok(ret)
  }

  pub fn on_error(&self) {}

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }
}
