mod testmod1;
mod testmod2;

// crate : Root of the project
// super: one module above

use crate::testmod2::t3 as mod2t3;

fn main() {
    testmod2::t3();
    testmod1::testmod2::t2();
    mod2t3();
}
