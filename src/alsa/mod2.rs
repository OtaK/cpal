pub extern crate alsa;

use self::alsa::{
    card::Card,
    pcm::{
        PCM,
        HwParams
    },
    Direction
};
use std::vec::IntoIter;

use ::{SupportedFormat, FormatsEnumerationError};

pub type SupportedInputFormats = IntoIter<SupportedFormat>;
pub type SupportedOutputFormats = IntoIter<SupportedFormat>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Device(PCM);

impl Device {
    pub fn name(&self) -> String {
        self.0.info().unwrap().get_name().unwrap().into()
    }

    fn supported_formats(&self, direction: Direction) -> Result<IntoIter<SupportedFormat>, FormatsEnumerationError> {
        let hwparams = HwParams::any(&self.0).map_err(|_| FormatsEnumerationError::DeviceNotAvailable)?;
        let formats = hwparams.get_supported_formats();
        Ok(formats.into_iter().map(|alsa_format| {
            SupportedFormat {

            }
        }).collect())
    }

    pub fn supported_output_formats(&self) -> Result<SupportedOutputFormats, FormatsEnumerationError> {
        self.supported_formats(Direction::Playback)
    }

    pub fn supported_input_formats(&self) -> Result<SupportedInputFormats, FormatsEnumerationError> {
        self.supported_formats(Direction::Capture)
    }
}