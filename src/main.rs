enum MediaPlayer {
    AudioPlayer,
    MediaAdapter,
}

impl MediaPlayer {
    fn play(&self, audio_type: &str, filename: &str) {
        match self {
            MediaPlayer::AudioPlayer => self.basic_play(audio_type, filename),
            MediaPlayer::MediaAdapter => self.advanced_play(audio_type, filename),
        }
    }
    fn basic_play(&self, audio_type: &str, filename: &str) {
        match audio_type {
            "mp3" => println!("Playing mp3 file. Name: {}", filename),
            "vlc" | "mp4" => {
                MediaPlayer::MediaAdapter.play(audio_type, filename);
            }
            _ => println!("Invalid media. {} format not supported.", audio_type),
        }
    }

    fn advanced_play(&self, audio_type: &str, filename: &str) {
        let amplayer = match audio_type {
            "vlc" => AdvancedMediaPlayer::VlcPlayer,
            _ => AdvancedMediaPlayer::Mp4Player,
        };
        amplayer.play(filename);
    }
}

enum AdvancedMediaPlayer {
    VlcPlayer,
    Mp4Player,
}

impl AdvancedMediaPlayer {
    fn play(&self, filename: &str) {
        match self {
            AdvancedMediaPlayer::Mp4Player => println!("Playing mp4 file. Name: {}", filename),
            AdvancedMediaPlayer::VlcPlayer => println!("Playing vlc file. Name: {}", filename),
        }
    }
}

fn main() {
    let audioplayer = MediaPlayer::AudioPlayer;

    audioplayer.play("mp3", "Islands in the Stream.mp3");
    audioplayer.play("mp4", "Home Alone 32.mp4");
    audioplayer.play("vlc", "The Matrix 15.vlc");
    audioplayer.play("avi", "Avengers: Regeneration");
}
