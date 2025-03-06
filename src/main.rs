use dl_who::err;
use dl_who::download;
use log::info;
use std::env;


#[tokio::main(flavor = "current_thread")]
async fn main() {

    let args: Vec<_> = env::args_os().collect();
    match download(args).await
    {
        Ok(_) => info!("Done!"),
        Err(e) => err::report_error(e),
    };
}