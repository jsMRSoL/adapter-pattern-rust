#![allow(dead_code)]
trait MediaPlayer {
    fn play(&mut self, audio_type: &str, filename: &str);
}

trait AdvancedMediaPlayer {
    fn play_vlc(&self, filename: &str);
    fn play_mp4(&self, filename: &str);
}

struct VlcPlayer;
impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self, filename: &str) {
	println!("Playing vlc file. Name: {}", filename);
    }
    fn play_mp4(&self, filename: &str) {
	()
    }
}

struct Mp4Player;
impl AdvancedMediaPlayer for Mp4Player {
    fn play_vlc(&self, filename: &str) {
	()
    }
    fn play_mp4(&self, filename: &str) {
	println!("Playing mp4 file. Name: {}", filename);
    }
}

struct MediaAdapter<'a> {
    advanced_media_player: Option<&'a dyn AdvancedMediaPlayer>,
}
impl<'a> MediaAdapter<'a> {
    fn new(audio_type: &str) -> Self {
	let amplayer: Option<&'a dyn AdvancedMediaPlayer> = if audio_type == "vlc" {
	    Some(&VlcPlayer)
	} else if audio_type == "mp4" {
	    Some(&Mp4Player)
	} else {
	    None
	};
	Self {
	    advanced_media_player: amplayer,
	}
    }
}
impl<'a> MediaPlayer for MediaAdapter<'a> {
    fn play(&mut self, audio_type: &str, filename: &str) {
	match audio_type {
	    "vlc" => self.advanced_media_player.unwrap().play_vlc(filename),
	    "mp4" => self.advanced_media_player.unwrap().play_mp4(filename),
	    _ => (),
	}
    }
}

struct AudioPlayer;
impl<'a> MediaPlayer for AudioPlayer {
    fn play(&mut self, audio_type: &str, filename: &str) {
	match audio_type {
	    "mp3" => println!("Playing mp3 file. Name: {}", filename),
	    "vlc" | "mp4" => {
		let media_adapter = &mut MediaAdapter::new(audio_type);
		media_adapter.play(audio_type, filename);
	    }
	    _ => println!("Invalid media. {} format not supported.", audio_type),
	}
    }
}
fn main() {
    let mut audioplayer = AudioPlayer;

    audioplayer.play("mp3", "Islands in the Stream.mp3");
    audioplayer.play("mp4", "Home Alone 32.mp4");
    audioplayer.play("vlc", "The Matrix 15.vlc");
    audioplayer.play("avi", "Avengers: Regeneration");
}
