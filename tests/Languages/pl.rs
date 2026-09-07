#![allow(non_snake_case)]

// Existing hand-written Polish tests (norms, degrees, relations, etc.)
mod pl;
mod number_sets;
mod unit_plurals;
mod preposition_case;

// Tests mirrored from the English suite, with expected speech localized to Polish.
mod ClearSpeak {
    mod functions;
    mod large_ops;
    mod menclose;
    mod mfrac;
    mod mroot;
    mod msup;
    mod sets;
    mod symbols_and_adornments;
    mod multiline;
}

mod SimpleSpeak {
    mod functions;
    mod large_ops;
    mod mfrac;
    mod msup;
    mod sets;
    mod geometry;
    mod linear_algebra;
    mod multiline;
    mod subscripts;
}
mod shared;
mod units;
mod chemistry;
mod alphabets;
mod definitions;
mod mtable;
