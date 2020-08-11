use pb::Message;
// Under the multi folder we have two folders /team and /user
use proto_multi::{
    // generates a team module
    team::svc::TeamRequest,
    // generates a user module
    user::svc::UserRequest,
};

fn main() {
    // What we do with these messages is largely irrelevant, we're just trying to show how proto
    // folder structure translates to Rust module structure

    let user_req = UserRequest {
        user_id: 1,
    };
    let team_req = TeamRequest {
        team_id: 1,
    };

    let user_bytes = user_req.serialize_to_vec();
    let team_bytes = team_req.serialize_to_vec();

    println!("Byte content is the same, but their meaning is different!");
    println!("User: {:X?}", user_bytes);
    println!("Team: {:X?}", team_bytes);
}
