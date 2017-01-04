use act::load_game;

fn main() {
    // Create a string containing our Act game
    let game_string = r#"
    {
       "rooms": [
           {
               "name": "start",
               "scene": "Im a starting room! Welcome to this example game.",
               "actions": [
                   {
                       "variant": "Move",
                       "fields": [
                           "Move to another room","example",""
                       ]
                   }
               ]
           },
           {
               "name": "example",
               "scene": "You enter an example room, with a big, triangular key in it. Theres also a door with a keyhole in triangular shape.",
               "actions": [
                   {
                       "variant": "PickUp",
                       "fields": [
                           "Pick the key up","TriangleKey",""
                       ]
                   },
                   {
                       "variant": "Move",
                       "fields": [
                           "Try to open the door","locked","TriangleKey"
                       ]
                   }
               ]
           }
           {
               "name": "locked",
               "scene": "You picked an item up and used it to open the door! This is the final room. Congratz!",
               "actions": [
                   {
                       "variant": "Move",
                       "fields": [
                           "Move to another room","example",""
                       ]
                   }
               ]
           }
       ]
    }
    "#
    // Load the game into a proper Game struct
    let game = load_game(game_string).unwrap();
    // Start the game
    game.play();
    // Profit!
}
