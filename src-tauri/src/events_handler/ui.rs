use crate::devices_intefaces::get_devices::Device;


#[tauri::command]
pub fn get_data_devices () -> String {
    let list = Device {};
    let res = list.serialize_list();
    return res;
    
}