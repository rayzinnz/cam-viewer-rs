use iced::{Element, Length, widget::container};
use iced_video_player::{Video, VideoPlayer};
use url::Url;

const RTSP_URL: &str = "rtsp://admin:your_password@192.168.1.34:554/user=admin_password=zprqvjbtwa2657!!EYVB_channel=1_stream=0a&protocol=unicast.sdp?real_stream";

fn main() -> iced::Result {
    iced::application(CamViewer::default, CamViewer::update, CamViewer::view)
        .title(CamViewer::title)
        .run()
}

struct CamViewer {
    video: Video,
}

// enum Message {
//     Tick,
// }

impl Default for CamViewer {
    fn default() -> Self {
        // Point this to your local video asset or a network stream URL
        let video_url = Url::parse(RTSP_URL)
            .expect("Failed to parse video URL");

        Self {
            video: Video::new(&video_url).expect("Failed to initialize video backend"),
        }
    }
}

impl CamViewer {
    fn title(&self) -> String {
        "RTSP Camera Viewer".to_string()
    }

    // Live feeds run continuously; no updates are needed unless adding controls
    fn update(&mut self, _message: ()) {}

    fn view(&self) -> Element<()> {
        // Instantiate the player layer targeting the live feed WGPU texture
        let player = VideoPlayer::new(&self.video);

        container(player)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

}

