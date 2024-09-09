use crate::create_json;
use crate::ipc_socket::DiscordIpcSocket;
use crate::models::events::EventReturn;
use crate::models::rpc_command::RPCCommand;
use crate::models::shared::User;
use crate::opcodes::OPCODES;
use crate::EventReceive;
use crate::Result;
use serde_json::json;
use uuid::Uuid;

// Environment keys to search for the Discord pipe

pub struct DiscordIpcConstruction {
  pub client: DiscordIpcClient,
  pub self_user: User,
}

#[allow(dead_code)]
#[allow(missing_docs)]
/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
pub struct DiscordIpcClient {
  /// Client ID of the IPC client.
  pub client_id: String,
  pub access_token: String,
  // Socket ref to the open socket
  pub socket: DiscordIpcSocket,
}

impl DiscordIpcClient {
  /// Creates a new `DiscordIpcClient`.
  ///
  /// ### Examples
  /// ```
  /// let ipc_client = DiscordIpcClient::new("<some client id>", "<some access token>")?;
  /// ```
  pub async fn setup(client_id: &str, access_token: &str) -> Result<DiscordIpcConstruction> {
    let socket = DiscordIpcSocket::new().await?;

    let mut client = Self {
      client_id: client_id.to_string(),
      access_token: access_token.to_owned(),
      socket,
    };

    // connect to client
    let self_user = client.connect().await?;

    // use the access_token to login
    client.login(access_token).await.ok();

    Ok(DiscordIpcConstruction { client, self_user })
  }

  /// Connects the client to the Discord IPC
  ///
  /// This method attempts to first establish a connection,
  /// and then sends a handshake
  pub async fn connect(&mut self) -> Result<User> {
    println!("Connecting to client...");

    self.send_handshake().await?;

    let (_opcode, payload) = self.socket.recv().await?;
    let mut self_user: Option<User> = None;

    // spooky line is not working
    let payload = serde_json::from_str(&payload)?;
    match payload {
      EventReturn::Ready { data } => {
        println!("Connected to discord and got ready event!");
        self_user = Some(data.user);
      }
      _ => {
        println!("Could not connect to discord...");
      }
    }

    Ok(self_user.unwrap())
  }

  /// Handshakes the Discord IPC.
  ///
  /// Returns an `Err` variant if sending the handshake failed.
  async fn send_handshake(&mut self) -> Result<()> {
    self
      .socket
      .send(
        &json!({
          "v": 1,
          "client_id": self.client_id
        })
        .to_string(),
        OPCODES::Handshake as u8,
      )
      .await?;

    // // TODO: Return an Err if the handshake is rejected
    // NOTE: this prolly shouldnt be done here as we dont want to consume messages here
    // self.recv()?;

    Ok(())
  }

  /// Send auth
  ///
  /// This method sends the auth token to the IPC.
  ///
  /// Returns an `Err` variant if sending the handshake failed.
  pub async fn login(&mut self, access_token: &str) -> Result<()> {
    let nonce = Uuid::new_v4().to_string();

    // TODO: move this to a struct and call send_cmd
    self
      .socket
      .send(
        &json!({
          "cmd": "AUTHENTICATE",
          "args": {
            "access_token": access_token
          },
          "nonce": nonce
        })
        .to_string(),
        OPCODES::Frame as u8,
      )
      .await?;

    self.socket.recv().await?;

    Ok(())
  }

  /// send a json string payload to the socket
  pub async fn emit_string(&mut self, payload: String) -> Result<()> {
    self
      .socket
      .send(&payload, OPCODES::Frame as u8)
      .await
      .unwrap();
    Ok(())
  }

  /// send a json string payload to the socket
  pub async fn emit_command(&mut self, command: &RPCCommand) -> Result<()> {
    let mut command_json = command.to_json()?;
    let json_string = &create_json(&mut command_json)?;
    self
      .socket
      .send(json_string, OPCODES::Frame as u8)
      .await
      .unwrap();
    Ok(())
  }

  pub async fn handler<F>(&mut self, func: F)
  where
    F: Fn(EventReceive) + Send + Sync + 'static,
  {
    let mut socket_clone = self.socket.clone();
    tokio::spawn(async move {
      loop {
        let (_opcode, payload) = socket_clone.recv().await.unwrap();

        match serde_json::from_str::<EventReceive>(&payload) {
          Ok(e) => {
            // TODO: give the consumer a ready event so they can sub to events
            func(e);
          }
          Err(err) => {
            println!("Unable to deserialize {:?}", err);
          }
        }
      }
    });
  }
}
