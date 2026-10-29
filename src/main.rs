use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use iced::widget::{self, Column, Text};
use iced::{executor, Element, Application, Settings, Theme, window};

const RTSP_URL: &str = "rtsp://admin:your_password@192.168.1.34:554/user=admin_password=v4HXkn4qbU_channel=1_stream=0a&protocol=unicast.sdp?real_stream";

static FRAME_STATE: OnceLock<Arc<Mutex<Option<Vec<u8>>>>> = OnceLock::new();

fn frame_state() -> &'static Arc<Mutex<Option<Vec<u8>>>> {
    FRAME_STATE.get_or_init(|| Arc::new(Mutex::new(None)))
}

struct CamViewer {
    image: Option<iced::image::Handle>,
    status: String,
}

enum Message {
    Tick,
}

impl Application for CamViewer {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Renderer = iced::Renderer;
    type Cache = ();

    fn new(_cache: ()) -> (Self, Settings<Message>) {
        (
            Self {
                image: None,
                status: "Connecting to camera...".to_string(),
            },
            Settings {
                window: window::Settings {
                    size: iced::Size::new(1280.0, 720.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    }

    fn title(&self) -> String {
        "RTSP Camera Viewer".to_string()
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Tick => {
                let state = frame_state();
                if let Ok(guard) = state.lock() {
                    if let Some(ref png_bytes) = *guard {
                        self.image =
                            Some(iced::image::Handle::from_bytes(png_bytes.clone()));
                        self.status = "Streaming".to_string();
                    }
                }
                iced::Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content: Element<Message> = if let Some(ref img) = self.image {
            widget::image(img.clone()).into()
        } else {
            Text::new(&self.status).into()
        };

        Column::with_children(vec![content])
            .padding(10)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_millis(33)).map(|_| Message::Tick)
    }
}

fn run_gstreamer(state: Arc<Mutex<Option<Vec<u8>>>>) {
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

    let pipeline = match gstreamer::parse_launch(&pipeline_desc) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to create pipeline: {}", e);
            return;
        }
    };

    let sink = match pipeline.by_name("sink") {
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
    };

    if let Err(e) = pipeline.set_state(gstreamer::State::Playing) {
        eprintln!("Failed to start pipeline: {}", e);
        return;
    }

    loop {
        match sink.pull_sample() {
            gstreamer::FlowSuccess::Ok(sample) => {
                if let Some(buffer) = sample.buffer() {
                    if let Some(map_info) = buffer.map_readable() {
                        let width = 640u32;
                        let height = 480u32;
                        let data = map_info.data();

                        let mut png_bytes = Vec::new();
                        let mut encoder =
                            image::codecs::png::PngEncoder::new(&mut png_bytes);
                        if encoder
                            .encode(data, width, height, image::ColorType::Rgba8)
                            .is_ok()
                        {
                            if let Ok(mut guard) = state.lock() {
                                *guard = Some(png_bytes);
                            }
                        }
                    }
                }
            }
            gstreamer::FlowSuccess::Eos => break,
            _ => {
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }

    let _ = pipeline.set_state(gstreamer::State::Null);
}

fn main() {
    let state = frame_state().clone();

    std::thread::spawn(move || {
        run_gstreamer(state);
    });

    CamViewer::run();
}
