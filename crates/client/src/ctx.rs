use crate::socket::ws::ClientSocket;

use anyhow::Result;
use generust_example_project_core::profile::UserProfile;
use generust_example_project_core::RequestMessage;
use std::rc::Rc;
use std::sync::RwLock;
use uuid::Uuid;
use web_sys::{Document, Window};

#[derive(Debug)]
pub(crate) struct ClientContext {
  window: Window,
  document: Document,
  socket: ClientSocket,
  connection_id: Option<Uuid>,
  profile: UserProfile
}

impl ClientContext {
  pub(crate) fn new() -> Result<Rc<RwLock<ClientContext>>> {
    let binary = true;
    let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("Can't find [window]"))?;
    let document = window.document().ok_or_else(|| anyhow::anyhow!("Can't find [document]"))?;
    let loc = &document.location().ok_or_else(|| anyhow::anyhow!("Can't find [location]"))?;
    let url = loc.href().map_err(|_| anyhow::anyhow!("Can't find [href]"))?;
    let socket = ClientSocket::new(&url, binary)?;
    let profile = UserProfile::default();

    let rc = Rc::new(RwLock::new(ClientContext {
      window,
      document,
      socket,
      connection_id: None,
      profile
    }));

    crate::socket::ws_events::wire_socket(Rc::clone(&rc));
    crate::socket::ws_events::on_load(&Rc::clone(&rc).read().unwrap())?;

    debug!("[{}] has started", generust_example_project_core::APPNAME);

    Ok(rc)
  }

  pub(crate) fn document(&self) -> &Document {
    &self.document
  }

  pub(crate) fn socket(&self) -> &ClientSocket {
    &self.socket
  }

  pub(crate) fn user_profile(&self) -> &UserProfile {
    &self.profile
  }

  pub(crate) fn on_connected(&mut self, connection_id: Uuid, profile: UserProfile, binary: bool) {
    self.socket.set_binary(binary);
    self.connection_id = Some(connection_id);
    self.profile = profile;
  }

  pub(crate) fn on_open(&self) -> Result<()> {
    Ok(())
  }

  pub(crate) fn send(&self, rm: RequestMessage) {
    self.socket.send(rm);
  }

  pub(crate) fn on_event(&mut self, t: &str, k: &str, v: &str) -> Result<()> {
    crate::event_handler::EventHandler::handle(self, t, k, v)
  }

  pub(crate) fn on_error(&self) -> Result<()> {
    Ok(())
  }

  pub(crate) fn on_close(&self) -> Result<()> {
    Ok(())
  }
}
