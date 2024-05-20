extern crate webbrowser;

use std::io;

fn main() -> io::Result<()> {
    let mut a = 0;
while a < 10{
    let tosunurl = "https://www.youtube.com/channel/UCkXehvE_6URhadvt_W-Jsmw";

    webbrowser::open(tosunurl)?;
    a +=1;
}
    Ok(())
}