use std::sync::Arc;

use hhmmss::Hhmmss;
use iced::time::{self, Duration, Instant, seconds};
use iced::widget::{Container, column, container, row, text};
use iced::window;
use iced::{Element, Length, Subscription, Task};
use iced_video_player::{Video, VideoPlayer};
use log::*;
use url::Url;

use crate::camera::{CameraInfo, get_camera_info, sync_camera_time};
use crate::network::scan_subnet;

mod camera;
mod network;

fn main() -> iced::Result {
    helper_lib::setup_logger(LevelFilter::Debug, None, "cam_viewer_rs", "cosmic_text,html5ever,iced,naga,sctk,tracing,wgpu_core,wgpu_hal,winit");

    iced::application(CamViewer::init, CamViewer::update, CamViewer::view)
        .subscription(CamViewer::subscription)
        .title(CamViewer::title)
        .run()
}

#[derive(Clone, Debug)]
struct VideoInfo {
    serial_no: String,
    ip: String,
    url: String,
    title: String,
    last_frame_instant: Instant,
    video: Arc<Option<Video>>,
    empty_slot_text: String,
}

impl Default for VideoInfo {
    fn default() -> Self {
        Self {
            serial_no: String::new(),
            ip: String::new(),
            url: String::new(),
            title: String::new(),
            last_frame_instant: Instant::now(),
            video: None.into(),
            empty_slot_text: String::new(),
        }
    }
}

struct CamViewer {
    video_infos: Vec<VideoInfo>,
    last_redraw_requested: Instant,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(()),
    OnNewFrame(Instant, String),
    OnCamError(String, String),
    RedrawRequested(Instant),
    GotCameraInfo(Vec<CameraInfo>),
    CameraConnected(VideoInfo),
}

impl Default for CamViewer {
    fn default() -> Self {
        trace!("check");
        
        let video_infos = vec![
            // cam 1, front door
            VideoInfo {
                serial_no: "769667d01ebf619a".to_string(),
                ip: String::new(),
                url: String::new(),
                title: String::new(),
                last_frame_instant: Instant::now(),
                video: None.into(),
                empty_slot_text: String::from("Starting up..."),
            },
            // cam 2, driveway
            VideoInfo {
                serial_no: "d1762ae4470a9485".to_string(),
                ip: String::new(),
                url: String::new(),
                title: String::new(),
                last_frame_instant: Instant::now(),
                video: None.into(),
                empty_slot_text: String::from("Starting up..."),
            },
            // cam 3, back
            VideoInfo {
                serial_no: "7f03583068bf0695".to_string(),
                ip: String::new(),
                url: String::new(),
                title: String::new(),
                last_frame_instant: Instant::now(),
                video: None.into(),
                empty_slot_text: String::from("Starting up..."),
            },
            // cam 4, back high
            VideoInfo {
                serial_no: "7fb52730db914b51hbr3".to_string(),
                ip: String::new(),
                url: String::new(),
                title: String::new(),
                last_frame_instant: Instant::now(),
                video: None.into(),
                empty_slot_text: String::from("Starting up..."),
            },
        ];

        Self {
            video_infos: video_infos,
            last_redraw_requested: Instant::now(),
        }
    }
}

impl CamViewer {
    fn title(&self) -> String {
        "RTSP Camera Viewer".to_string()
    }

    fn init() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::done(Message::Tick(())), // <--- Triggers at exactly 0 seconds
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick(_) => {
                //ignore tick if off screen
                if self.last_redraw_requested.elapsed() > Duration::from_secs(1) {
                    debug!("Window is off-screen, so do not process tick.");
                    return Task::none();
                }
                //remove any disconnected
                for mut_video_info in self.video_infos.iter_mut() {
                    if mut_video_info.video.is_some() && mut_video_info.last_frame_instant.elapsed() > Duration::from_secs(5) {
                        warn!("{} ({}) at {} has not had a frame update for at least 5 seconds. Disconnecting.", mut_video_info.title, mut_video_info.serial_no, mut_video_info.ip);
                        mut_video_info.video = None.into();
                    }
                }
                //check for cameras
                Task::perform(get_camera_infos(), Message::GotCameraInfo)
            },
            Message::GotCameraInfo(camera_infos) => {
                let mut tasks = Vec::new();
                for mut_video_info in self.video_infos.iter_mut() {
                    if mut_video_info.video.is_none() {
                        //find returned value
                        if let Some(camera_info) = camera_infos.iter().find(|ci| ci.serial_no==mut_video_info.serial_no) {
                            let url = format!("rtsp://{}:554/user={}_password={}_channel=1_stream=0a&protocol=unicast.sdp?real_stream&protocols=tcp",
                                camera_info.ip,
                                camera::USER,
                                camera::PASSWORD,
                            );
                            mut_video_info.url = url;
                            mut_video_info.ip = camera_info.ip.to_string();
                            mut_video_info.title = camera_info.title.to_string();

                            //now get video, Task per camera for parallel connection
                            tasks.push(Task::perform(connect_camera(mut_video_info.clone()), Message::CameraConnected));
                            
                        } else {
                            error!("Found camera with serial_no = '{}', but this has not been explicitly defined as a camera.", mut_video_info.serial_no);
                        }
                    }
                }
                Task::batch(tasks)
            },
            Message::CameraConnected(video_info) => {
                //link returned video to self
                if let Some(mut_video_info) = self.get_mut_video_info_from_serial_no(&video_info.serial_no) {
                    if video_info.video.is_some() {
                        mut_video_info.video = video_info.video.clone();
                    }

                    if mut_video_info.title.is_empty() {
                        mut_video_info.empty_slot_text = format!("No video for cam: {}\nNot found on network for {}",
                            mut_video_info.serial_no,
                            mut_video_info.last_frame_instant.elapsed().hhmmss(),
                        );
                    } else {
                        mut_video_info.empty_slot_text = format!("No video for cam: {}\n{} at {} not seen for {}",
                            mut_video_info.serial_no,
                            mut_video_info.title,
                            mut_video_info.ip,
                            mut_video_info.last_frame_instant.elapsed().hhmmss()
                        );
                    }
                }

                Task::none()
            },
            Message::OnNewFrame(now, serial_no) => {
                if let Some(mut_video_info) = self.get_mut_video_info_from_serial_no(&serial_no) {
                    mut_video_info.last_frame_instant = now;
                };
                Task::none()
            },
             Message::OnCamError(serial_no, e) => {
                if let Some(mut_video_info) = self.get_mut_video_info_from_serial_no(&serial_no) {
                    error!("Error from stream on cam {} ({}, {}) Error: {}", mut_video_info.ip, mut_video_info.title, serial_no, e);
                };
                Task::none()
            },
            Message::RedrawRequested(now) => {
                self.last_redraw_requested = now;
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut player_conts: Vec<Container<Message>> = Vec::new();
        for video_info in &self.video_infos {
            let video_deref_arc = &*video_info.video;
            if let Some(video) = video_deref_arc {
                let player = VideoPlayer::new(video)
                    .on_new_frame(Message::OnNewFrame(Instant::now(), video_info.serial_no.clone()))
                    .on_error(|e| Message::OnCamError(video_info.serial_no.clone(), e.to_string()));
                let player_cont: Container<Message> =             container(player)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                        .into();
                player_conts.push(player_cont);
            } else {
                //space filler
                player_conts.push(container(text(&video_info.empty_slot_text))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
                );
            }
        }
        
        let num_cols = 2;

        // Break the vector into chunks of length x (columns)
        let rows = player_conts
            .into_iter()
            .fold(Vec::new(), |mut acc: Vec<Vec<Element<'_, Message>>>, item| {
                if acc.last().map_or(true, |row| row.len() == num_cols) {
                    acc.push(vec![item.into()]);
                } else {
                    acc.last_mut().unwrap().push(item.into());
                }
                acc
            });

        // Build rows containing columns
        let grid_column = rows.into_iter().fold(column!().spacing(10), |col, row_items| {
            let r = row(row_items).spacing(10);
            col.push(r)
        });

        grid_column.into()

    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            //check every 10 seconds for camera changes
            time::every(seconds(10)).map(|_| Message::Tick(())),

            window::frames().map(Message::RedrawRequested),
        ])
    }

    fn get_mut_video_info_from_serial_no(&mut self, serial_no: &str) -> Option<&mut VideoInfo> {
        self.video_infos.iter_mut().find(|vi| vi.serial_no==serial_no)
    }

}

async fn get_camera_infos() -> Vec<CameraInfo> {
    let mut camera_infos:Vec<CameraInfo> = Vec::new();
    //add any new
    let camera_ips = scan_subnet();
    info!("Found {} RTSP camera(s) on port 554:", camera_ips.len());
    
    // let rt = Runtime::new().unwrap();
    for ip in camera_ips {
        // match rt.block_on(get_camera_info(&ip)) {
        match get_camera_info(&ip).await {
            Ok(camera_info) => {
                info!("camera_info: {:?}", camera_info);
                camera_infos.push(camera_info);
            },
            Err(e) => {
                error!("Could not get camera info for {ip}: {e}")
            }
        }
    }

    camera_infos
}

async fn connect_camera(video_info: VideoInfo) -> VideoInfo {
    if let Ok(_) = sync_camera_time(&video_info.ip).await {
        info!("Time on camera {} ({}) synced with computer", video_info.ip, video_info.title);
    }
    info!("Connecting to {}", video_info.url);
    match Video::new(&Url::parse(&video_info.url).unwrap()) {
        Ok(mut video) => {
            video.set_volume(0.0);
            VideoInfo { 
                serial_no: video_info.serial_no.to_string(),
                video: Some(video).into(),
                ..Default::default()
            }
        },
        Err(e) => {
            error!("Could not connect to {}\nError: {}", video_info.url, e);
            VideoInfo { 
                serial_no: video_info.serial_no.to_string(),
                ..Default::default()
            }
        }
    }
}
