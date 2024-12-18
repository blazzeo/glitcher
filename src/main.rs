use nokhwa::{
    self,
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

fn main() {
    let index = CameraIndex::Index(0);
    let req = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = Camera::new(index, req).unwrap();

    let _ = camera.open_stream();
    let mut frames = vec![];
    loop {
        let frame = camera.frame();
        frames.push(frame);
    }
}
