use windows::Win32::Media::*;
use windows::Win32::Devices::*;
use core::fmt::Result;

fn test() -> Result {
    // Create an instance of an AudioGraph which represents a collection of interconnected audio nodes that can generate, process, and consume audio data.
    let audio_graph = AudioGraph::create_async(AudioGraphCreationMode::Default);

    // Get all audio capture devices.
    let audio_capture_devices = DeviceInformation::find_all_async(DeviceClass::AudioCapture)?;

    // Select the first audio capture device.
    let audio_capture_device = audio_capture_devices.get_at(0)?;

    // Create an instance of an AudioDeviceInputNode which represents a node that can be used to record audio.
    let audio_device_input_node = audio_graph.create_device_input_node_async_with_device_id(audio_capture_device.id()?)?;

    // Set the output format of the node to PCM with 48kHz sample rate, 2 channels, and 16 bits per sample.
    let encoding_properties = AudioEncodingProperties::create_pcm(48000u32, 2u32, 16u32)?;
    audio_device_input_node.set_encoding_properties(encoding_properties)?;

    // Start recording audio.
    audio_graph.start()?;

    Ok(())
}