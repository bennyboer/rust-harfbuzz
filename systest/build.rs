extern crate ctest;

use std::env;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    // Get the include paths from harfbuzz-sys or pkg-config.
    if let Some(path) = &env::var_os("DEP_HARFBUZZ_INCLUDE") {
        // This comes from a static build in harfbuzz-sys.
        cfg.include(path);
    }

    // Include the header files where the C APIs are defined.
    cfg.header("hb.h").header("hb-ot.h").header("hb-aat.h");

    if cfg!(target_vendor = "apple") {
        cfg.header("hb-coretext.h");
    }

    // Skip structs that are empty on the Rust side.
    cfg.skip_struct(|s| {
        s == "hb_language_impl_t"
            || s == "hb_blob_t"
            || s == "hb_unicode_funcs_t"
            || s == "hb_set_t"
            || s == "hb_face_t"
            || s == "hb_font_t"
            || s == "hb_font_funcs_t"
            || s == "hb_buffer_t"
            || s == "hb_map_t"
            || s == "hb_shape_plan_t"
            || s == "hb_ot_math_kern_entry_t"
            || s == "hb_draw_funcs_t"
    });

    // FIXME: I'm not sure why these functions must be skipped.
    cfg.skip_fn(|s| s == "hb_buffer_append" || s == "hb_ot_math_get_glyph_kernings");

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    if let Ok(bindings) = &env::var("DEP_HARFBUZZ_BINDINGS") {
        cfg.generate(&bindings, "all.rs");
    } else {
        cfg.generate("../harfbuzz-sys/src/bindings.rs", "all.rs");
    }
}
