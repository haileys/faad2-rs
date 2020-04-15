use std::os::raw::{c_char, c_int, c_long, c_uchar, c_ushort, c_ulong, c_void};

type NeAACDecHandle = *mut c_void;

#[repr(C)]
#[allow(non_snake_case)]
pub struct mp4AudioSpecificConfig {
    /* Audio Specific Info */
    objectTypeIndex: c_uchar,
    samplingFrequencyIndex: c_uchar,
    samplingFrequency: c_ulong,
    channelsConfiguration: c_uchar,

    /* GA Specific Info */
    frameLengthFlag: c_uchar,
    dependsOnCoreCoder: c_uchar,
    coreCoderDelay: c_ushort,
    extensionFlag: c_uchar,
    aacSectionDataResilienceFlag: c_uchar,
    aacScalefactorDataResilienceFlag: c_uchar,
    aacSpectralDataResilienceFlag: c_uchar,
    epConfig: c_uchar,

    sbr_present_flag: c_char,
    forceUpSampling: c_char,
    downSampledSBR: c_char,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct NeAACDecConfiguration {
    defObjectType: c_uchar,
    defSampleRate: c_ulong,
    outputFormat: c_uchar,
    downMatrix: c_uchar,
    useOldADTSFormat: c_uchar,
    dontUpSampleImplicitSBR: c_uchar,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct NeAACDecFrameInfo
{
    bytesconsumed: c_ulong,
    samples: c_ulong,
    channels: c_uchar,
    error: c_uchar,
    samplerate: c_ulong,

    /* SBR: 0: off, 1: on; upsample, 2: on; downsampled, 3: off; upsampled */
    sbr: c_uchar,

    /* MPEG-4 ObjectType */
    object_type: c_uchar,

    /* AAC header type; MP4 will be signalled as RAW also */
    header_type: c_uchar,

    /* multichannel configuration */
    num_front_channels: c_uchar,
    num_side_channels: c_uchar,
    num_back_channels: c_uchar,
    num_lfe_channels: c_uchar,
    channel_position: [c_uchar; 64],

    /* PS: 0: off, 1: on */
    ps: c_uchar,
}

extern "C" {
    pub fn NeAACDecGetErrorMessage(
        errcode: c_uchar,
    ) -> *const c_uchar;

    pub fn NeAACDecGetCapabilities() -> c_ulong;

    pub fn NeAACDecOpen() -> NeAACDecHandle;

    pub fn NeAACDecGetCurrentConfiguration(
        decoder: NeAACDecHandle,
    ) -> *const NeAACDecConfiguration;

    pub fn NeAACDecSetConfiguration(
        decoder: NeAACDecHandle,
        config: *const NeAACDecConfiguration,
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
        buffer: *mut c_uchar,
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
        buffer: *mut c_uchar,
        buffer_size: c_ulong,
    ) -> *mut c_void;

    pub fn NeAACDecDecode2(
        decoder: NeAACDecHandle,
        hInfo: *mut NeAACDecFrameInfo,
        buffer: *mut c_uchar,
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
