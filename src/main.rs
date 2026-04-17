use std::{collections::HashSet, env, ops, path::Path, process};
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
    // if args.len() < 2 {
    //     eprintln!("Usage: rust_app <path_to_file>");
    //     process::exit(1);
    // }
    // let file_path = &args[1];

    // let file_name = "MP Replay v101.103.38337.0 @2026.02.21 170707 (7).aoe2record";
    let file_name = "MP Replay v101.103.39862.0 @2026.04.16 212054 (8).aoe2record";
    let folder_path = "C:/Source/Data/FVDLeaderboard/analyze/";
    let file_path = format!(
        // "C:/Source/Data/FVDLeaderboard/replays/savedchapter/{}",
        "C:/Users/tomek/Games/Age of Empires 2 DE/76561198073652290/savegame/{}",
        // "C:/Source/Repos/FVDLeaderBoards/ReplayParser/aoe2rec/files/{}",
        file_name
    );

    let ops = parse_operations(file_path.to_string(), 200000, true);
    let output_path = Path::new(folder_path).join("ops.json");
    std::fs::write(output_path, ops.join("\n")).unwrap();

    // let game_info = parse(file_path.to_string());
    // let json = serde_json::to_string_pretty(&game_info).unwrap();

    // println!("{}", json);
}

pub fn parse(path: String) -> GameInfo {
    let savegame = Savegame::from_file(Path::new(&path)).unwrap();

    let s = savegame.get_summary();
    let teams = s.teams;

    // Collect all chat messages as plain Strings using the `LenString`'s `Serialize` impl.
    // We can't access `text.value` (it's private), so serialize to `serde_json::Value`
    // and extract the string produced by the `LenString` serializer.
    // let chat_texts: Vec<String> = savegame
    //     .operations
    //     .iter()
    //     .filter_map(|op| {
    //         if let Operation::Chat { text, .. } = op {
    //             match serde_json::to_value(text) {
    //                 Ok(serde_json::Value::String(s)) => Some(s),
    //                 Ok(other) => Some(other.to_string()),
    //                 Err(_) => Some(format!("{:?}", text)),
    //             }
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // Print collected chat messages to console
    // for (i, txt) in chat_texts.iter().enumerate() {
    //     println!("chat[{}]: {}", i, txt);
    // }

    

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

pub fn parse_operations(path: String, max_operations: usize, ignore_sync: bool) -> Vec<String> {
    let savegame = Savegame::from_file(Path::new(&path)).unwrap();

    savegame
        .operations
        .iter()
        .take(max_operations)
        .filter_map(|op| match op {
            Operation::Chat { text, .. } => {
                let text_value = serde_json::to_value(text)
                    .ok()
                    .and_then(|value| value.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| format!("{:?}", text));
                Some(format!("{{ \"Chat\": {} }}", text_value))
            }
            // Operation::Chat { .. } => {
            //     let json = serde_json::to_string(op)
            //         .unwrap_or_else(|_| format!("{:?}", op));
            //     Some(json)
            // }
            Operation::Viewlock { .. } => None,
            Operation::Sync { .. } if ignore_sync => None,
            Operation::Sync { .. } => {
                let json = serde_json::to_string(op)
                    .unwrap_or_else(|_| format!("{:?}", op));
                Some(json)
            }
            Operation::Action { .. } => {
                let json = serde_json::to_string(op)
                    .unwrap_or_else(|_| format!("{:?}", op));
                Some(json)
            }
            // Operation::Action { length, action_data, world_time, chap } => {
            //     let action_json = serde_json::json!({
            //         "length": length,
            //         "action_data": action_data,
            //         "world_time": world_time,
            //         "chap": chap,
            //     });
            //     Some(format!("Action {}", action_json.to_string()))
            // }
            other => Some(format!("{:?}", other)),
            // other => None,
        })
        .collect()
}

fn action_data_name(action_data: &ActionData) -> String {
    let debug = format!("{:?}", action_data);
    debug
        .split('{')
        .next()
        .unwrap_or(&debug)
        .trim()
        .to_string()
}

// fn get_player_actions(savegame: &Savegame, player_id: u8) -> Vec<ActionData> {
//     savegame.operations
//         .iter()
//         .filter_map(|op| {
//             if let Operation::Action { action_data, .. } = op {
//                 if action_data.player_id() == Some(player_id) {
//                     Some(action_data)
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         })
//         .cloned()            
//         .collect()
// }

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
            | ActionData::Transform { player_id, .. }
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
            | ActionData::Achievements { player_id, .. }
            | ActionData::Game { player_id,..} => Some(*player_id),
        }
    }
}
