// A dangling else binds to the right-most if.
if (true) if (false) assert(false); else assert(true); // expect: good
if (false) if (true) assert(false); else assert(false);
