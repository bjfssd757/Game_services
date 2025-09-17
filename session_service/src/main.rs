pub mod proto {
    tonic::include_proto!("session");
}

use proto::session_server::{Session, SessionServer};
use serde::{Serialize, Deserialize};
use tonic::Status;


#[derive(Serialize, Deserialize, Debug)]
enum ServerStatusProto {
    Stopped,
    Running,
}

#[derive(Serialize, Deserialize, Debug)]
enum GameStatusProto {
    InGame,
    InLobby,
}

#[derive(Serialize, Deserialize, Debug)]
struct SessionInfoProto {
    id: i64,
    dedicated_server_id: i64,
    game_status: GameStatusProto,
    leader_id: i64,
    in_session_players_id: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SessionsList {
    sessions: Vec<SessionInfoProto>,
}

#[derive(Debug, Default)]
struct SessionService;

impl SessionService {
    async fn load_all_sessions() -> Result<SessionsList, Box<dyn std::error::Error>> {
        Ok(SessionsList {
            sessions: vec![
                SessionInfoProto {
                    dedicated_server_id: 1,
                    id: 1,
                    leader_id: 2,
                    game_status: GameStatusProto::InLobby,
                    in_session_players_id: 1,
                }
            ]
        })
    }
}

#[tonic::async_trait]
impl Session for SessionService {
    async fn create_session(
        &self,
        req: tonic::Request<proto::CreateSessionRequest>
    ) -> Result<tonic::Response<proto::CreateSessionResponse>, Status> {
        Ok(())
    }

    async fn delete_session(
        &self,
        req: tonic::Request<proto::DeleteSessionRequest>
    ) -> Result<tonic::Response<proto::DeleteSessionResponse>, Status> {
        Ok(())
    }

    async fn update_session(
        &self,
        req: tonic::Request<proto::UpdateSessionRequest>
    ) -> Result<tonic::Response<proto::UpdateSessionResponse>, Status> {
        Ok(())
    }

    async fn join_session(
        &self,
        req: tonic::Request<proto::JoinSessionRequest>
    ) -> Result<tonic::Response<proto::JoinSessionResponse>, Status> {
        Ok(())
    }

    async fn leave_session(
        &self,
        req: tonic::Request<proto::LeaveSessionRequest>
    ) -> Result<tonic::Response<proto::LeaveSessionResponse>, Status> {
        Ok(())
    }

    async fn invite_session(
        &self,
        req: tonic::Request<proto::InviteSessionRequest>
    ) -> Result<tonic::Response<proto::InviteSessionResponse>, Status> {
        Ok(())
    }

    async fn accept_invite(
        &self,
        req: tonic::Request<proto::AcceptInviteRequest>
    ) -> Result<tonic::Response<proto::AcceptInviteResponse>, Status> {
        Ok(())
    }

    async fn reject_invite(
        &self,
        req: tonic::Request<proto::RejectInviteRequest>
    ) -> Result<tonic::Response<proto::RejectInviteResponse>, Status> {
        Ok(())
    }
}

fn main() {
    
}
