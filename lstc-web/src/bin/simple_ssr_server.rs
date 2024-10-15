use std::error::Error;
use std::path::PathBuf;

use bytes::Bytes;
use clap::Parser;
use futures::stream::{self, Stream, StreamExt};
use simple_ssr::app::{App, AppProps};
use warp::Filter;

type BoxedError = Box<dyn Error + Send + Sync + 'static>;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[structopt(short, long)]
    dir: PathBuf,
}

async fn render(
    before: String,
    after: String,
) -> Box<dyn Stream<Item = Result<Bytes, BoxedError>> + Send> {
    let api_base_url = "".to_string();
    let renderer = yew::ServerRenderer::<App>::with_props(|| AppProps { api_base_url });

    Box::new(
        stream::once(async move { before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { after }))
            .map(|m| Result::<_, BoxedError>::Ok(m.into())),
    )
}

#[tokio::main]
async fn main() {
    let opts = Opt::parse();

    let html = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (before, html) = html.split_once("<body>").unwrap();
    let (html, after) = html.split_once("</body>").unwrap();

    let before = format!("{}<body>", before);
    let after = format!("</body>{}", after);

    let html = warp::path::end().then(move || {
        let before = before.clone();
        let after = after.clone();
        println!(">>> {}", before);
        println!(">>> {}", after);

        async move { warp::reply::html(render(before, after).await) }
    });

    let routes = html.or(warp::fs::dir(opts.dir));

    println!("You can view the website at: http://localhost:8080/");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
