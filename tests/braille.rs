#![allow(non_snake_case)]

mod common;

mod braille {
    mod Nemeth {
        mod rules;
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

    mod Finnish {
        mod spec;
    }

    mod Vietnam {
        mod vi;
    }
}

