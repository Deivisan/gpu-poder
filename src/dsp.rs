use std::fs;
use std::path::PathBuf;

const DSP_DEVICE: &str = "/dev/adsprpc-smd";

#[derive(Debug)]
pub struct DspAccess {
    device_path: PathBuf,
    has_adsp: bool,
    has_cdsp: bool,
}

impl DspAccess {
    pub fn new() -> Result<Self, String> {
        let path = PathBuf::from(DSP_DEVICE);
        let has_adsp = path.exists();

        let cdsp_path = PathBuf::from("/dev/adsprpc-smd-secure");
        let has_cdsp = cdsp_path.exists();

        if !has_adsp && !has_cdsp {
            return Err("Nenhum dispositivo DSP encontrado".to_string());
        }

        Ok(DspAccess {
            device_path: path,
            has_adsp,
            has_cdsp,
        })
    }

    pub fn has_adsp(&self) -> bool {
        self.has_adsp
    }

    pub fn has_cdsp(&self) -> bool {
        self.has_cdsp
    }

    pub fn get_dsp_info(&self) -> Result<serde_json::Value, String> {
        let mut info = serde_json::json!({
            "adsp_available": self.has_adsp,
            "cdsp_available": self.has_cdsp,
        });

        if self.has_adsp {
            info["adsp_device"] = serde_json::json!("/dev/adsprpc-smd");
        }

        if self.has_cdsp {
            info["cdsp_device"] = serde_json::json!("/dev/adsprpc-smd-secure");
        }

        Ok(info)
    }

    pub fn list_dsp_devices(&self) -> Vec<String> {
        let mut devices = Vec::new();

        if self.has_adsp {
            devices.push("/dev/adsprpc-smd".to_string());
        }
        if self.has_cdsp {
            devices.push("/dev/adsprpc-smd-secure".to_string());
        }

        devices
    }

    pub fn is_compute_dsp_available(&self) -> bool {
        self.has_cdsp
    }
}

pub fn check_dsp_available() -> bool {
    PathBuf::from(DSP_DEVICE).exists()
}
