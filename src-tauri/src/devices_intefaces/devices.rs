use std::option::Option;
use std::ptr;
use windows::Win32::Media::Audio::{
    eCapture, eConsole, eRender, IAudioCaptureClient, IAudioClient, IAudioRenderClient,
    IAudioSessionControl, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
    AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK, DEVICE_STATE_ACTIVE,
    DEVICE_STATE_NOTPRESENT, DEVICE_STATE_UNPLUGGED, WAVEFORMATEX, WAVE_FORMAT_PCM,
};
use windows::Win32::System::Com::{
    CoCreateGuid, CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL, CLSCTX_INPROC_SERVER, CoInitializeEx, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
};

use windows::core::{Interface, GUID};

const BUFFER_DURATION: i64 = 100; // Buffer duration in milliseconds

// static COM_INITIALIZED: Once = Once::new();

// fn initialize_com() {
//     COM_INITIALIZED.call_once(|| unsafe {
//         CoInitialize(Some(ptr::null_mut()));
//     });
// }


pub fn wire() {
   
    // initialize_com();
    // unsafe { CoInitialize(Some(ptr::null_mut())).unwrap() };
    unsafe {
        let result = CoInitializeEx(Some(ptr::null_mut()), COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
        let started = match result {
            Ok(t) => t,
            Err(error) => panic!("Problem: {:?}", error)
        };
         println!("{:?}", started);
        let enumerator =
            CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL)
                .unwrap();
        let mic = enumerator
            .GetDefaultAudioEndpoint(eCapture, eConsole)
            .unwrap();
        
        let audio_client: *mut IAudioClient = ptr::null_mut();
        let audio_client_ref = { &mut *audio_client };
        let mix_format: WAVEFORMATEX = WAVEFORMATEX {
            wFormatTag: WAVE_FORMAT_PCM as u16,
            nChannels: 2,
            nSamplesPerSec: 44100,
            nAvgBytesPerSec: 176400,
            nBlockAlign: 4,
            wBitsPerSample: 16,
            cbSize: 0,
        };
       
        // let hr = audio_client_ref.Initialize(
        //     AUDCLNT_SHAREMODE_SHARED,
        //     AUDCLNT_STREAMFLAGS_LOOPBACK,
        //     0,
        //     0,
        //     &mut mix_format as *mut WAVEFORMATEX,
        //     Some(ptr::null()),
        // );

    }

    unsafe { CoUninitialize() };
}
