#![allow(non_snake_case)]
#![allow(clippy::needless_return)]

mod common;

mod braille {
    mod Nemeth {
        mod rules;
        mod other;
        mod chemistry;
        mod SRE_Nemeth72;
        mod SRE_NemethBase;
        mod AataNemeth;
    }
    mod UEB {
        mod iceb;
        mod other;
    }

    mod CMU {
        mod once;
    }

    mod Vietnam {
        mod vi;
    }

    mod Russian {
        mod russian;
    }

    mod Swedish {
        mod swedish;
    }

    mod Polish {
        mod polish;
    }

    mod LaTeX {
        mod augenbit;
        mod other;
    }

    mod ASCIIMath {
        mod augenbit;
        mod other;
    }

    mod French {
        mod french;
        mod chemistry;
        mod other;
    }
}

