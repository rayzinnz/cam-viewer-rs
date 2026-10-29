use std::sync::{Arc, Mutex, OnceLock};
// use std::time::Duration;

use glib::object::Cast;
use gstreamer::prelude::{ElementExt, GstBinExt};
use iced::widget::{Column, Text};
use iced::{Element, Subscription};
use iced::time::{self, Duration};
use image::ImageEncoder;

const RTSP_URL: &str = "rtsp://admin:your_password@192.168.1.34:554/user=admin_password=v4HXkn4qbU_channel=1_stream=0a&protocol=unicast.sdp?real_stream";

static FRAME_STATE: OnceLock<Arc<Mutex<Option<String>>>> = OnceLock::new();

fn frame_state() -> &'static Arc<Mutex<Option<String>>> {
    FRAME_STATE.get_or_init(|| Arc::new(Mutex::new(None)))
}

struct CamViewer {
    image_path: Option<String>,
    status: String,
}

enum Message {
    Tick,
}

impl Default for CamViewer {
    fn default() -> Self {
        Self {
            image_path: None,
            status: "Connecting to camera...".to_string(),
        }
    }
}

impl CamViewer {
    fn title(&self) -> String {
        "RTSP Camera Viewer".to_string()
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Tick => {
                let state = frame_state();
                if let Ok(guard) = state.lock() {
                    if let Some(ref path) = *guard {
                        self.image_path = Some(path.clone());
                        self.status = "Streaming".to_string();
                    }
                }
                iced::Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = if let Some(ref path) = self.image_path {
            iced::widget::image(path).into()
        } else {
            Text::new(&self.status).into()
        };

        Column::with_children(vec![content])
            .padding(10)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(33))
            .map(|_| Message::Tick)
    }
}

fn run_gstreamer(state: Arc<Mutex<Option<String>>>) {
    if let Err(e) = gstreamer::init() {
        eprintln!("Failed to initialize GStreamer: {}", e);
        return;
    }

    let pipeline_desc = format!(
        "rtspsrc location='{}' latency=100 \
         ! rtph264depay \
         ! avdec_h264 \
         ! videoconvert \
         ! videoscale \
         ! video/x-raw,format=RGBA,width=640,height=480 \
         ! appsink name=sink emit-signals=false max-buffers=1 drop=true",
        RTSP_URL
    );

    let pipeline = match gstreamer::parse::launch(&pipeline_desc) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to create pipeline: {}", e);
            return;
        }
    };

    let sink = match pipeline.dynamic_cast_ref::<gstreamer::Bin>() {
        Some(bin) => match bin.by_name("sink") {
            Some(s) => match s.downcast::<gstreamer_app::AppSink>() {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Failed to downcast to AppSink");
                    return;
                }
            },
            None => {
                eprintln!("Failed to find appsink in pipeline");
                return;
            }
        }
        None => {
            eprintln!("Failed to dynamic_cast_ref pipline to bin");
            return;
        }
    };

    if let Err(e) = pipeline.set_state(gstreamer::State::Playing) {
        eprintln!("Failed to start pipeline: {}", e);
        return;
    }

    let frame_path = std::env::temp_dir().join("cam_viewer_frame.png");
    let frame_path_str = frame_path.to_string_lossy().to_string();

    loop {
        if let Ok(sample) = sink.pull_sample() {
            if let Some(buffer) = sample.buffer() {
                if let Ok(map_info) = buffer.map_readable() {
                    let width = 640u32;
                    let height = 480u32;
                    let data = map_info.as_slice();

                    let mut png_bytes = Vec::new();
                    let encoder =
                        image::codecs::png::PngEncoder::new(&mut png_bytes);
                    if encoder
                        .write_image(data, width, height, image::ColorType::Rgba8.into())
                        .is_ok()
                    {
                        let _ = std::fs::write(&frame_path, &png_bytes);
                        if let Ok(mut guard) = state.lock() {
                            *guard = Some(frame_path_str.clone());
                        }
                    }
                }
            }
        } else {
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    // let _ = pipeline.set_state(gstreamer::State::Null);
}

fn main() -> iced::Result {
    let state = frame_state().clone();

    std::thread::spawn(move || {
        run_gstreamer(state);
    });

    iced::application(CamViewer::default, CamViewer::update, CamViewer::view)
        .title(CamViewer::title)
        .subscription(CamViewer::subscription)
        .run()
}
