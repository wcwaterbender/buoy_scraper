
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://wavecast.com/buoys/").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    let document = Html::parse_document(&body);

    let selector = Selector::parse("img").unwrap();

    let mut images = vec!();

    //get the wave plot images only
    for element in document.select(&selector) {
        match element.value().attr("src") {
            Some(x) if x.contains("plot_wave") => images.push(x),
            _x => drop(_x),
        }
    }
    let mut counter = 1;

    //make requests for the byte data and save as images locally
    for imgurl in images {
        let img_bytes = reqwest::get(imgurl).await?;
        let bytes = img_bytes.bytes().await?;
        match image::load_from_memory(&bytes){
            Ok(x) => {
                let mut filename = String::from("imgs/buoy_");
                filename+=&counter.to_string();
                filename += &String::from(".png");
                match x.save(filename){
                    Ok(_) => println!("Saved Image"),
                    Err(_) => println!("failed to save img"),
                };
                counter+=1;
            },
            Err(x) => println!("no image {}",x),
        }
    }
    Ok(())
}
