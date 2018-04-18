use model::{FromRawObject, RawObject, ValueError};
use model::de;

#[derive(Debug)]
pub struct MainSong {
    main_song_id: u8,
    name: &'static str,
    artist: &'static str,
}

#[derive(Debug, FromRawObject)]
pub struct NewgroundSong {
    #[raw_data(index = 1)]
    song_id: u64,

    #[raw_data(index = 2)]
    name: String,

    // Index 3 has unknown usage

    #[raw_data(index = 4)]
    artist: String,

    #[raw_data(index = 5)]
    filesize: u64,

    // Index 6 has unknown usage

    #[raw_data(index = 7)]
    alt_artist: String,

    #[raw_data(index = 8, deserialize_with="de::int_to_bool")]
    banned: bool,

    // Index 9 is unused

    #[raw_data(index = 10)]
    link: String,

}

impl MainSong {
    fn new(main_song_id: u8, name: &'static str, artist: &'static str) -> MainSong {
        MainSong {
            main_song_id,
            name,
            artist,
        }
    }
}

lazy_static! {
    pub static ref MAIN_SONGS: [MainSong; 21] = [
        MainSong::new(0, "Stereo Madness", "ForeverBound"),
        MainSong::new(1, "Back on Track", "DJVI"),
        MainSong::new(2, "Polargeist", "Step"),
        MainSong::new(3, "Dry Out", "DJVI"),
        MainSong::new(4, "Base after Base", "DJVI"),
        MainSong::new(5, "Can't Let Go", "DJVI"),
        MainSong::new(6, "Jumper", "Waterflame"),
        MainSong::new(7, "Time Machine", "Waterflame"),
        MainSong::new(8, "Cycles", "DJVI"),
        MainSong::new(9, "xStep", "DJVI"),
        MainSong::new(10, "Clutterfunk", "Waterflame"),
        MainSong::new(11, "Theory of Everything", "DJ-Nate"),
        MainSong::new(12, "Electroman ADventures", "Waterflame"),
        MainSong::new(13, "Clubstep", "DJ-Nate"),
        MainSong::new(14, "Electrodynamix", "DJ-Nate"),
        MainSong::new(15, "Hexagon Force", "Waterflame"),
        MainSong::new(16, "Blast Processing", "Waterflame"),
        MainSong::new(17, "Theory of Everything 2", "DJ-Nate"),
        MainSong::new(18, "Geometrical Dominator", "Waterflame"),
        MainSong::new(19, "Deadlocked", "F-777"),
        MainSong::new(20, "Fingerdash", "MDK"),
    ];

    pub static ref UNKNOWN: MainSong = MainSong::new(0xFF, "The song was added after the release of GDCF you're using", "Please either update to the newest version, or bug stadust about adding the new songs");
}