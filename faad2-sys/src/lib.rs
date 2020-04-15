pub use std::os::raw::{c_char, c_int, c_long, c_uchar, c_ushort, c_ulong, c_void};

pub type NeAACDecHandle = *mut c_void;

pub const FAAD_FMT_16BIT: c_uchar = 1;
pub const FAAD_FMT_24BIT: c_uchar = 2;
pub const FAAD_FMT_32BIT: c_uchar = 3;
pub const FAAD_FMT_FLOAT: c_uchar = 4;
pub const FAAD_FMT_DOUBLE: c_uchar = 5;

#[repr(C)]
#[allow(non_snake_case)]
pub struct mp4AudioSpecificConfig {
    /* Audio Specific Info */
    pub objectTypeIndex: c_uchar,
    pub samplingFrequencyIndex: c_uchar,
    pub samplingFrequency: c_ulong,
    pub channelsConfiguration: c_uchar,

    /* GA Specific Info */
    pub frameLengthFlag: c_uchar,
    pub dependsOnCoreCoder: c_uchar,
    pub coreCoderDelay: c_ushort,
    pub extensionFlag: c_uchar,
    pub aacSectionDataResilienceFlag: c_uchar,
    pub aacScalefactorDataResilienceFlag: c_uchar,
    pub aacSpectralDataResilienceFlag: c_uchar,
    pub epConfig: c_uchar,

    pub sbr_present_flag: c_char,
    pub forceUpSampling: c_char,
    pub downSampledSBR: c_char,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct NeAACDecConfiguration {
    pub defObjectType: c_uchar,
    pub defSampleRate: c_ulong,
    pub outputFormat: c_uchar,
    pub downMatrix: c_uchar,
    pub useOldADTSFormat: c_uchar,
    pub dontUpSampleImplicitSBR: c_uchar,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct NeAACDecFrameInfo
{
    pub bytesconsumed: c_ulong,
    pub samples: c_ulong,
    pub channels: c_uchar,
    pub error: c_uchar,
    pub samplerate: c_ulong,

    /* SBR: 0: off, 1: on; upsample, 2: on; downsampled, 3: off; upsampled */
    pub sbr: c_uchar,

    /* MPEG-4 ObjectType */
    pub object_type: c_uchar,

    /* AAC header type; MP4 will be signalled as RAW also */
    pub header_type: c_uchar,

    /* multichannel configuration */
    pub num_front_channels: c_uchar,
    pub num_side_channels: c_uchar,
    pub num_back_channels: c_uchar,
    pub num_lfe_channels: c_uchar,
    pub channel_position: [c_uchar; 64],

    /* PS: 0: off, 1: on */
    pub ps: c_uchar,
}

extern "C" {
    pub fn NeAACDecGetErrorMessage(
        errcode: c_uchar,
    ) -> *const c_char;

    pub fn NeAACDecGetCapabilities() -> c_ulong;

    pub fn NeAACDecOpen() -> NeAACDecHandle;

    pub fn NeAACDecGetCurrentConfiguration(
        decoder: NeAACDecHandle,
    ) -> *mut NeAACDecConfiguration;

    pub fn NeAACDecSetConfiguration(
        decoder: NeAACDecHandle,
        config: *mut NeAACDecConfiguration,
    ) -> c_uchar;

    /* Init the library based on info from the AAC file (ADTS/ADIF) */
    pub fn NeAACDecInit(
        decoder: NeAACDecHandle,
        buffer: *mut c_uchar,
        buffer_size: c_ulong,
        samplerate: *mut c_ulong,
        channels: *mut c_uchar,
    ) -> c_ulong;

    /* Init the library using a DecoderSpecificInfo */
    pub fn NeAACDecInit2(
        decoder: NeAACDecHandle,
        buffer: *const c_uchar,
        SizeOfDecoderSpecificInfo: c_ulong,
        samplerate: *mut c_ulong,
        channels: *mut c_uchar,
    ) -> c_uchar;

    /* Init the library for DRM */
    pub fn NeAACDecInitDRM(
        decoder: *mut NeAACDecHandle,
        samplerate: c_ulong,
        channels: c_uchar,
    ) -> c_uchar;

    pub fn NeAACDecPostSeekReset(
        decoder: NeAACDecHandle,
        frame: c_long,
    );

    pub fn NeAACDecClose(
        decoder: NeAACDecHandle,
    );

    pub fn NeAACDecDecode(
        decoder: NeAACDecHandle,
        hInfo: *mut NeAACDecFrameInfo,
        buffer: *const c_uchar,
        buffer_size: c_ulong,
    ) -> *mut c_void;

    pub fn NeAACDecDecode2(
        decoder: NeAACDecHandle,
        hInfo: *mut NeAACDecFrameInfo,
        buffer: *const c_uchar,
        buffer_size: c_ulong,
        sample_buffer: *mut *mut c_void,
        sample_buffer_size: c_ulong,
    ) -> *mut c_void;

    pub fn NeAACDecAudioSpecificConfig(
        pBuffer: *mut c_uchar,
        buffer_size: c_ulong,
        mp4ASC: *mut mp4AudioSpecificConfig,
    ) -> c_uchar;

    pub fn NeAACDecGetVersion(id: *mut *const c_char, copyright: *mut *const c_char) -> c_int;
}
