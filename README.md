# fvd_replay_parser_cli

## how to run
open command line and run `fvd_replay_parser_cli.exe "path to .aoe2record file"`  
or `fvd_replay_parser_cli "path to .aoe2record file"`

## example output
```json
{
  "timestamp": 1760378100,
  "player_infos": [
    {
      "player_number": 1,
      "profile_id": 111111,
      "resigned": false,
      "builder": true,
      "name": "X",
      "actions": 25,
      "chapter": false
    },
    {
      "player_number": 2,
      "profile_id": 222222,
      "resigned": false,
      "builder": true,
      "name": "gsnwizard999",
      "actions": 54,
      "chapter": false
    },
    {
      "player_number": 3,
      "profile_id": 333333,
      "resigned": false,
      "builder": true,
      "name": "sosna",
      "actions": 6,
      "chapter": false
    },
    {
      "player_number": 4,
      "profile_id": 444444,
      "resigned": false,
      "builder": true,
      "name": "donut",
      "actions": 52,
      "chapter": false
    },
    {
      "player_number": 5,
      "profile_id": 555555,
      "resigned": false,
      "builder": true,
      "name": "friends",
      "actions": 41,
      "chapter": true
    },
    {
      "player_number": 6,
      "profile_id": 666666,
      "resigned": false,
      "builder": true,
      "name": "RoarRicheAh",
      "actions": 25,
      "chapter": false
    },
    {
      "player_number": 7,
      "profile_id": 777777,
      "resigned": true,
      "builder": false,
      "name": "羊弟",
      "actions": 2,
      "chapter": false
    },
    {
      "player_number": 8,
      "profile_id": 888888,
      "resigned": false,
      "builder": true,
      "name": "max-elios",
      "actions": 15,
      "chapter": false
    }
  ]
}
```