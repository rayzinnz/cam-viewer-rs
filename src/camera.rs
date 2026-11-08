use anyhow::{Result, bail};
use chrono::{DateTime, Local};
use dvrip_rs::{Authentication, Connection, DVRIPCam, SystemInfo};
use log::{debug, error};

pub const USER:&str = "admin";
pub const PASSWORD:&str = "zprqvjbtwa2657!!EYVB";
// const RTSP_URL: &str = "rtsp://192.168.1.34:554/user=admin_password=zprqvjbtwa2657!!EYVB_channel=1_stream=0a&protocol=unicast.sdp?real_stream";

#[derive(Debug, Clone)]
pub struct CameraInfo {
	pub serial_no: String,
	pub ip: String,
	pub title: String,
	pub time: DateTime<Local>,
}

pub async fn get_camera_info(ip: &str) -> Result<CameraInfo> {
    let mut cam = DVRIPCam::new(ip);
    if let Err(e) = cam.connect(tokio::time::Duration::from_secs(10)).await {
		let _ = cam.logout().await;
		let _ = cam.close().await;
        bail!("Error on cam.connect to {ip}: {e}")
    }
    for itry in 0..3 {
		debug!("cam {} try login attempt {}", ip, itry+1);
		if let Err(e) = cam.login(USER, PASSWORD).await {
			if itry==2 {
				bail!("Cam {ip} login error: {e}")
			}
			let _ = cam.logout().await;
			let _ = cam.close().await;
			error!("Login to {ip} failed: {e}\ntry again");
		} else {
			break;
		}
	}
    let info = cam.get_system_info().await?;
	let serial_no: String;
    if let Some(serial_no2) = info.get("SerialNo") {
		serial_no = serial_no2.as_str().unwrap().to_string();
	} else {
		let _ = cam.logout().await;
		let _ = cam.close().await;
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

	let _ = cam.logout().await;
	let _ = cam.close().await;

	Ok(CameraInfo { serial_no: serial_no, ip: ip.to_string(), title: title, time: time })
}

pub async fn sync_camera_time(ip: &str) -> Result<()> {
    let mut cam = DVRIPCam::new(ip);
    if let Err(e) = cam.connect(tokio::time::Duration::from_secs(10)).await {
		let _ = cam.logout().await;
		let _ = cam.close().await;
        bail!("Error on cam.connect to {ip}: {e}")
    }
    if let Err(e) = cam.login(USER, PASSWORD).await {
		let _ = cam.logout().await;
		let _ = cam.close().await;
        bail!("Cam {ip} login error: {e}")
    }
    if let Err(e) = cam.set_time(Some(Local::now())).await {
		let _ = cam.logout().await;
		let _ = cam.close().await;
        bail!("Cam {ip} set time error: {e}")
    }

	let _ = cam.logout().await;
	let _ = cam.close().await;

	Ok(())
}
