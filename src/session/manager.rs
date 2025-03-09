use chrono::{DateTime, Utc};
use rand::random;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};

pub type SessionId = String;

pub struct SessionData {
    task: tokio::task::JoinHandle<()>,
    last_ping: Option<DateTime<Utc>>,
    last_ping_success: Option<DateTime<Utc>>,
}

impl SessionData {
    fn from_task(task: tokio::task::JoinHandle<()>) -> Self {
        SessionData { task, last_ping: None, last_ping_success: None }
    }
}

pub enum SessionManagerCmd {
    AddSession(SessionId),
    RemoveSession(SessionId),
}

pub struct SessionManager {
    pub task_queue_tx: tokio::sync::mpsc::Sender<SessionManagerCmd>,
    pub task_queue_rx: tokio::sync::mpsc::Receiver<SessionManagerCmd>,
    sessions: HashMap<SessionId, SessionData>,
}

impl SessionManager {
    pub fn new() -> SessionManager {
        let (tx, rx) = tokio::sync::mpsc::channel(1024);
        SessionManager { sessions: HashMap::new(), task_queue_tx: tx, task_queue_rx: rx }
    }

    pub async fn run(&mut self) {
        self.start().await
    }

    async fn start(&mut self) {
        while let Some(cmd) = self.task_queue_rx.recv().await {
            match cmd {
                SessionManagerCmd::AddSession(id) => {
                    if !self.sessions.contains_key(&id) {
                        log::info!("Adding new session: {}", id);
                        let id_closure = id.clone();
                        let task = tokio::spawn(async {
                            Self::start_session(id_closure).await;
                        });
                        self.sessions.insert(id, SessionData::from_task(task));
                    } else {
                        log::info!("Session already exists: {}", id);
                    }
                }
                SessionManagerCmd::RemoveSession(id) => {
                    if let Some(session) = self.sessions.get_mut(&id) {
                        log::info!("Removing session: {}", id);
                        session.task.abort();
                        self.sessions.remove(&id);
                    } else {
                        log::info!("Cannot remove missing session: {}", id);
                    }
                }
            }
        }
    }

    async fn start_session(id: SessionId) {
        let config = Config::default();
        let client = Client::new(&config).expect("Failed to create surge_ping client");
        let ip = Ipv4Addr::from_str(&id).expect("Session id is not valid ipv4");

        let mut pinger = client.pinger(IpAddr::from(ip), PingIdentifier(random())).await;
        pinger.timeout(Duration::from_secs(1));

        loop {
            match pinger.ping(PingSequence(0), &[]).await {
                Err(e) => {
                    log::error!("NO PING response from {}: {}", ip, e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok((_, rtt)) => {
                    log::info!("PING response from {} ({:?})", ip, rtt);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}