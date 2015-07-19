struct Song;
struct AudioClip;

enum SongKind {
    Hit,
    Flop,
}

trait Singer {
    fn sing(&mut self, s: Song) -> AudioClip;
}

trait SongWriter {
    fn compose(&mut self, kind: SongKind) -> Song;
}

struct Artist;

impl Singer for Artist {
    fn sing(&mut self, _s: Song) -> AudioClip {
        AudioClip
    }
}

impl SongWriter for Artist {
    fn compose(&mut self, _kind: SongKind) -> Song {
        Song
    }
}

#[test]
/// Traits allow implementations of its own interface, using calls to itself.
/// See the `Iterator` trait for a perfect example.
fn it_works() {
}

#[test]
fn artist() {
    let mut a = Artist;
    let song = a.compose(SongKind::Hit);
    a.sing(song);
}


