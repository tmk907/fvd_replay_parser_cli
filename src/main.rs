use std::{env, process, collections::HashSet, path::Path};
use aoe2rec::actions::ActionData;
use serde::Serialize;
use aoe2rec::{*};

#[derive(Serialize)]
pub struct GameInfo {
    timestamp: i32,
    player_infos: Vec<PlayerInfo>,
}

#[derive(Serialize)]
pub struct PlayerInfo {
    player_number: i32,
    profile_id: i32,
    resigned: bool,
    builder: bool,
    name: String,
    actions: i32,
    chapter: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Expect the file path as the first argument
    if args.len() < 2 {
        eprintln!("Usage: rust_app <path_to_file>");
        process::exit(1);
    }
    let file_path = &args[1];

    // let file_name = "AgeIIDE_Replay_422884788.aoe2record";
    // let file_path = format!(
    //     "C:/Source/Data/FVDLeaderboard/replays/{}",
    //     // "C:/Users/tomek/Games/Age of Empires 2 DE/76561198073652290/savegame/{}",
    //     file_name
    // );

    let game_info = parse(file_path.to_string());
    let json = serde_json::to_string_pretty(&game_info).unwrap();

    println!("{}", json);
}

pub fn parse(path: String) -> GameInfo {
    let savegame = Savegame::from_file(Path::new(&path)).unwrap();

    let s = savegame.get_summary();
    let teams = s.teams;

    let mut wall_players:Vec<u8> = savegame.operations
            .iter()
            .filter_map(|op| {
                if let Operation::Action { action_data, .. } = op {
                    match action_data {
                         actions::ActionData::Wall { player_id,.. } => Some(player_id),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<HashSet<&u8>>()
            .into_iter()
            .cloned()            
            .collect();
    wall_players.sort();

    // let actions: Vec<ActionData> = get_player_actions(&savegame, 8);
    // let c =  count_player_actions(&savegame, 8);

    let resigned:Vec<u8> = savegame.operations
        .iter()
        .filter_map(|op| {
            if let Operation::Action { action_data, .. } = op {
                match action_data {
                        actions::ActionData::Resign { player_id,.. } => Some(player_id),
                    _ => None,
                }
            } else {
                None
            }
        })
        .cloned()            
        .collect();

    let chapter:Vec<u8> = savegame.operations
        .iter()
        .filter_map(|op| {
            if let Operation::Action { action_data, .. } = op {
                match action_data {
                        actions::ActionData::Chapter { player_id,.. } => Some(player_id),
                    _ => None,
                }
            } else {
                None
            }
        })
        .cloned()            
        .collect();

    let player_infos: Vec<PlayerInfo> = teams
        .iter()
        .filter_map(|team| team.players.first())
        .map(|team_player| PlayerInfo {
            player_number: team_player.info.player_number,
            profile_id: team_player.info.profile_id,
            resigned: resigned.contains(&(team_player.info.player_number.try_into().unwrap_or(100))),
            name: String::from(&team_player.info.name),
            builder: wall_players.contains(&(team_player.info.player_number.try_into().unwrap_or(100))),
            actions: count_player_actions(&savegame, team_player.info.player_number.try_into().unwrap_or(100)) as i32,
            chapter: chapter.contains(&(team_player.info.player_number.try_into().unwrap_or(100))),
        })
        .collect();

    let game_info = GameInfo {
        player_infos: player_infos,
        timestamp: savegame.zheader.timestamp,
    };

    return game_info;

}

fn get_player_actions(savegame: &Savegame, player_id: u8) -> Vec<ActionData> {
    savegame.operations
        .iter()
        .filter_map(|op| {
            if let Operation::Action { action_data, .. } = op {
                if action_data.player_id() == Some(player_id) {
                    Some(action_data)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .cloned()            
        .collect()
}

fn count_player_actions(savegame: &Savegame, player_id: u8) -> usize {
    savegame.operations
        .iter()
        .filter(|op| {
            if let Operation::Action { action_data, .. } = op {
                action_data.player_id() == Some(player_id)
            } else {
                false
            }
        })
        .count()
}

trait HasPlayerId {
    fn player_id(&self) -> Option<u8>;
}

impl HasPlayerId for ActionData {
    fn player_id(&self) -> Option<u8> {
        match self {
            ActionData::Interact { player_id, .. }
            | ActionData::Build { player_id, .. }
            | ActionData::Stop { player_id, .. }
            | ActionData::AiInteract { player_id, .. }
            | ActionData::Move { player_id, .. }
            | ActionData::Create { player_id, .. }
            | ActionData::AddAttribute { player_id, .. }
            | ActionData::GiveAttribute { player_id, .. }
            | ActionData::AiMove { player_id, .. }
            | ActionData::Resign { player_id, .. }
            | ActionData::Spec { player_id, .. }
            | ActionData::Waypoint { player_id, .. }
            | ActionData::Stance { player_id, .. }
            | ActionData::Guard { player_id, .. }
            | ActionData::Follow { player_id, .. }
            | ActionData::Patrol { player_id, .. }
            | ActionData::Formation { player_id, .. }
            | ActionData::Save { player_id, .. }
            | ActionData::AiWaypoint { player_id, .. }
            | ActionData::Chapter { player_id, .. }
            | ActionData::DeAttackMove { player_id, .. }
            | ActionData::DeUnknown35 { player_id, .. }
            | ActionData::DeUnknown37 { player_id, .. }
            | ActionData::Autoscout { player_id, .. }
            | ActionData::DeUnknown39 { player_id, .. }
            | ActionData::Unknown41 { player_id, .. }
            | ActionData::SwitchAttack { player_id, .. }
            | ActionData::Unknown44 { player_id, .. }
            | ActionData::Unknown45 { player_id, .. }
            | ActionData::AiCommand { player_id, .. }
            | ActionData::AiQueue { player_id, .. }
            | ActionData::Research { player_id, .. }
            | ActionData::Wall { player_id, .. }
            | ActionData::Delete { player_id, .. }
            | ActionData::AttackGround { player_id, .. }
            | ActionData::Tribute { player_id, .. }
            | ActionData::DeUnknown109 { player_id, .. }
            | ActionData::Repair { player_id, .. }
            | ActionData::Release { player_id, .. }
            | ActionData::Multiqueue { player_id, .. }
            | ActionData::ToggleGate { player_id, .. }
            | ActionData::Flare { player_id, .. }
            | ActionData::Order { player_id, .. }
            | ActionData::Queue { player_id, .. }
            | ActionData::Gatherpoint { player_id, .. }
            | ActionData::Sell { player_id, .. }
            | ActionData::Buy { player_id, .. }
            | ActionData::DropRelic { player_id, .. }
            | ActionData::TownBell { player_id, .. }
            | ActionData::BackToWork { player_id, .. }
            | ActionData::DeQueue { player_id, .. }
            | ActionData::DeUnknown130 { player_id, .. }
            | ActionData::DeUnknown131 { player_id, .. }
            | ActionData::DeUnknown135 { player_id, .. }
            | ActionData::DeUnknown140 { player_id, .. }
            | ActionData::DeUnknown196 { player_id, .. }
            | ActionData::Unknown104 { player_id, .. }
            | ActionData::Achievements { player_id, .. } => Some(*player_id),
            ActionData::Game(_) => None,
        }
    }
}
