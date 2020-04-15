static FAAD_SOURCE_DIR: &'static str = "faad2/libfaad";

static FAAD_SOURCES: &'static [&'static str] = &[
    "bits.c",
    "cfft.c",
    "common.c",
    "decoder.c",
    "drc.c",
    "error.c",
    "filtbank.c",
    "hcr.c",
    "huffman.c",
    "ic_predict.c",
    "is.c",
    "lt_predict.c",
    "mdct.c",
    "mp4.c",
    "ms.c",
    "output.c",
    "pns.c",
    "ps_dec.c",
    "ps_syntax.c",
    "pulse.c",
    "rvlc.c",
    "sbr_dct.c",
    "sbr_dec.c",
    "sbr_e_nf.c",
    "sbr_fbt.c",
    "sbr_hfadj.c",
    "sbr_hfgen.c",
    "sbr_huff.c",
    "sbr_qmf.c",
    "sbr_syntax.c",
    "sbr_tf_grid.c",
    "specrec.c",
    "syntax.c",
    "tns.c",
];

static FAAD_INCLUDE_DIR: &'static str = "faad2/include";
static FAAD_INCLUDE: &'static str = "neaacdec.h";

fn sources() -> Vec<String> {
    FAAD_SOURCES.iter()
        .map(|file| format!("{}/{}", FAAD_SOURCE_DIR, file))
        .collect()
}

fn main() {
    for src in sources() {
        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed={}", src);
    }

    println!("cargo:rerun-if-changed={}/{}", FAAD_INCLUDE_DIR, FAAD_INCLUDE);

    cc::Build::new()
        .warnings(false)
        .files(sources())
        .include(FAAD_INCLUDE_DIR)
        .include(FAAD_SOURCE_DIR)
        .define("HAVE_INTTYPES_H", "1")
        .define("HAVE_MEMCPY", "1")
        .define("HAVE_STDINT_H", "1")
        .define("HAVE_STDLIB_H", "1")
        .define("HAVE_STRCHR", "1")
        .define("STDC_HEADERS", "1")
        .define("PACKAGE_VERSION", "\"2.9.1\"")
        .compile("libfaad2.a");

    println!("cargo:rustc-link-lib=faad2");
}
