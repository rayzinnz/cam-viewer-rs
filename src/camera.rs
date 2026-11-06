
use anyhow::{Result, bail};
use chrono::{DateTime, Local};
use dvrip_rs::{Authentication, Connection, DVRIPCam, SystemInfo};

pub const USER:&str = "admin";
pub const PASSWORD:&str = "zprqvjbtwa2657!!EYVB";

pub struct CameraInfo {
	pub serial_no: String,
	pub title: String,
	pub time: DateTime<Local>,
}

pub async fn get_camera_info(ip: &str) -> Result<CameraInfo> {
    let mut cam = DVRIPCam::new(ip);
    if let Err(e) = cam.connect(tokio::time::Duration::from_secs(10)).await {
        bail!("Error on cam.connect to {ip}: {e}")
    }
    if let Err(e) = cam.login(USER, PASSWORD).await {
        bail!("Cam {ip} login error: {e}")
    }
    let info = cam.get_system_info().await?;
	let serial_no: String;
    if let Some(serial_no2) = info.get("SerialNo") {
		serial_no = serial_no2.as_str().unwrap().to_string();
	} else {
		bail!("SerialNo not found for cam {ip}")
	}

	let title: String;
	if let Ok(title_ok) = cam.get_channel_titles().await {
		title = title_ok.first().unwrap_or(&String::new()).to_string();
	} else {
		title = String::new();
	}

	let time = match cam.get_time().await {
		Ok(time) => time,
		Err(e) => bail!("Could not get cam {ip} time: {e}")
	};

	Ok(CameraInfo { serial_no: serial_no, title: title, time: time })
}

