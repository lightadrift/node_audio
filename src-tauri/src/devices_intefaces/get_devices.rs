// use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};
// use std::error::Error;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr::{self, null};
use std::slice;
// use windows::core::PWSTR;
use windows::Win32::Devices::Properties;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;

pub enum DeviceReturn {
    Name(String),
    Names(Vec<String>),
}

#[derive(Debug)]
pub struct Device {}

pub struct Devices {
    collection: IMMDeviceCollection,
    total_count: u32,
    next_item: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct ListOfDevices {
    pub name: String,
    pub id: String,
}

impl Device {
    fn name(&self, device: IMMDevice) -> String {
        unsafe {
            let property_store = device.OpenPropertyStore(STGM_READ).unwrap();
            let property_value = property_store
                .GetValue(&Properties::DEVPKEY_Device_FriendlyName as *const _ as *const _)
                .unwrap();
            let prop_variant = &property_value.Anonymous.Anonymous;
            let ptr_utf16 = *(&prop_variant.Anonymous as *const _ as *const *const u16);
            let mut len = 0;
            while *ptr_utf16.offset(len) != 0 {
                len += 1;
            }
            let name_slice = slice::from_raw_parts(ptr_utf16, len as usize);
            let name_os_string: OsString = OsStringExt::from_wide(name_slice);
            let name_string = match name_os_string.into_string() {
                Ok(string) => string,
                Err(os_string) => os_string.to_string_lossy().into(),
            };
            return name_string;
        }
    }
    pub fn get_list(&self, dataflow: EDataFlow, dwstatekask: u32) -> Vec<ListOfDevices> {
        let mut list_devices: Vec<ListOfDevices> = vec![];
        unsafe {
            let enumerate =
                CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL)
                    .unwrap();
            let result = enumerate.EnumAudioEndpoints(dataflow, dwstatekask).unwrap();
            let device_count = result.GetCount().unwrap();
            let mut n: u32 = 0;
            while n < device_count {
                let device = ListOfDevices {
                    name: self.name(result.Item(n).expect("não há nome")),
                    id: result
                        .Item(n)
                        .unwrap()
                        .GetId()
                        .unwrap()
                        .to_string()
                        .expect("error"),
                };
                if n == device_count {
                    break;
                }
                list_devices.push(device);
                n += 1;
            }
            CoUninitialize();
        }
        return list_devices;
    }

    pub fn serialize_list(&self) -> String {
        let list = self.get_list(eAll, DEVICE_STATE_ACTIVE);
        let json_list = serde_json::to_string(&list).expect("não foi possivel serializar");
        return json_list;
    }
}
