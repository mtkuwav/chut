#[derive(Debug, PartialEq)]
struct CueSheet {
    performer: Option<String>,
    title: Option<String>,
    file: Option<String>,
    tracks: Vec<Track>,
    catalog: Option<String>,
}

#[derive(Debug, PartialEq)]
struct Track {
    number: u32,
    title: String,
    index: Index,
}

#[derive(Debug, PartialEq)]
struct Index {
    number: u8,
    minute: u8,
    second: u8,
    frame: u8,
}

#[derive(Debug, PartialEq)]
struct Metadata {
    performer: Option<String>,
    title: Option<String>,
    songwriter: Option<String>,

}