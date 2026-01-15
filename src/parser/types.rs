#[derive(Debug, PartialEq)]
struct CueSheet {
    metadata: Metadata,
    file: Option<String>,
    tracks: Vec<Track>,
    catalog: Option<String>,
}

#[derive(Debug, PartialEq)]
struct Track {
    number: u8, // must be between 1 and 99
    pregap: Option<Index>,
    index: TrackIndex,
    flags: Option<Flags>,
    metadata: Metadata,
    postgap: Option<Index>,
}

#[derive(Debug, PartialEq)]
struct TrackIndex {
    number: u8,
    index: Index,
}

#[derive(Debug, PartialEq)]
struct Index {
    minute: u8,
    second: u8,
    frame: u8, // there are 75 frames per second
}

#[derive(Debug, PartialEq)]
/// Fields as told by from https://wyday.com/cuesharp/specification.php.
///
/// - `ARRANGER`: Name(s) of the arranger(s).
/// - `COMPOSER`: Name(s) of the composer(s).
/// - `DISC_ID`: Disc Identification information.
/// - `GENRE`: Genre Identification and Genre information.
/// - `ISRC`: ISRC Code of each track.
/// - `MESSAGE`: Message from the content provider and/or artist.
/// - `PERFORMER`: Name(s) of the performer(s).
/// - `SONGWRITER`: Name(s) of the songwriter(s).
/// - `TITLE`: Title of album name or Track Titles.
/// - `TOC_INFO`: Table of Content information.
/// - `TOC_INFO2`: Second Table of Content information.
/// - `UPC_EAN`: UPC/EAN code of the album.
/// - `SIZE_INFO`: Size information of the Block.
struct Metadata {
    /// Name(s) of the arranger(s).
    arranger: Option<String>,
    /// Name(s) of the composer(s).
    composer: Option<String>,
    /// Disc Identification information.
    disc_id: Option<String>,
    /// Genre Identification and Genre information.
    genre: Option<String>,
    /// ISRC Code of each track.
    isrc: Option<String>,
    /// Message from the content provider and/or artist.
    message: Option<String>,
    /// Name(s) of the performer(s).
    performer: Option<String>,
    /// Name(s) of the songwriter(s).
    songwriter: Option<String>,
    /// Title of album name or Track Titles.
    title: Option<String>,
    /// Table of Content information.
    toc_info: Option<String>,
    /// Second Table of Content information.
    toc_info2: Option<String>,
    /// UPC/EAN code of the album.
    upc_ean: Option<String>,
    /// Size information of the Block.
    size_info: Option<String>,
}

#[derive(Debug, PartialEq)]
/// Track special sub-code flags, rarely used today.
///
/// - `PRE`: **PRE**-emphasis enabled (audio tracks only)
/// - `DCP`: **D**igital **C**opy **P**ermitted
/// - `FourCH` *(4CH)*: **4** **CH**annel audio
/// - `SCMS`: **S**erial **C**opy **M**anagement **S**ystem
enum Flags {
    PRE,
    DCP,
    FourCH,
    SCMS
}