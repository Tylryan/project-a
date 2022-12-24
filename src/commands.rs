use std::path::PathBuf;
use glob::glob;

pub struct Commands { }

impl Commands 
{
    // add deck deck1
    pub fn add_deck(deck_name: String)    
    { 
        // Eventually will be able to add subdecks
        let config_path = ".";
        let mut decks_path  = PathBuf::from(format!("{config_path}/decks"));
        if !decks_path.exists() 
        { 
            std::fs::create_dir(&decks_path).unwrap()
        }
        decks_path.push(format!("{deck_name}.deck"));
        std::fs::File::create(&decks_path).unwrap();
    }
    // add subdeck deck1::deck2
    pub fn add_subdeck(decks_names: String) { todo!()}
    // rm deck deck1
    pub fn remove_deck(deck_name: String) 
    { 
        // Eventually will be able to add subdecks
        let config_path = ".";
        let mut deck_path  = PathBuf::from(format!("{config_path}/decks/{deck_name}.deck"));

        if deck_path.exists() 
        { 
            std::fs::remove_file(&deck_path).unwrap();
        }
    }
    pub fn list_decks(deck_path: &PathBuf) -> Vec<String>
    {
        let mut deck_names: Vec<String> = Vec::new();
        let deck_path  = deck_path
            .to_string_lossy()
            .to_string() + "/*";
        let deck_paths = glob(&deck_path).unwrap();

        for deck in deck_paths
        {
            let deck_name = deck.unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            deck_names.push(deck_name);
        }

        return deck_names;
    }

    pub fn edit_deck(deck_name: String) 
    {
        let config_path       = ".";
        let deck_path: String = format!("{config_path}/decks/{deck_name}.deck");
        let editor: String    = std::env::var("EDITOR").unwrap();
        let command: String   = format!("{editor} {deck_path}");
        std::process::Command::new("bash")
            .args(["-c", &command])
            .spawn()
            .unwrap()
            .wait().unwrap();
    }
    // rm subdeck deck1::deck2
    pub fn remove_subdeck(deck_name: String) { todo!() }
    // rn deck deck1 deck2
    pub fn rename_deck(deck_name: String) { todo!() }
    // rn subdeck deck1::deck2 deck1::deck3
    pub fn rename_subdeck(deck_name: String) { todo!() }
    // add card front::back deck1
    pub fn add_card(front: String, back: String, deck_name: String) { todo!() }
    // rm card front deck1
    pub fn remove_card(front: String, deck_name: String) { todo!() }
    // rn card a::b a::c deck1
    pub fn rename_card(front: String) { todo!() }
}
