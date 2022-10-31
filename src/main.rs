extern crate dotenv;
use cosm_orc::{
    config::{
        cfg::Config,
        key::{Key, SigningKey},
    },
    orchestrator::cosm_orc::CosmOrc,
};
use dotenv::dotenv;
use image::GenericImageView;
use palette::Srgb;

fn main() {
    dotenv().ok();

    let config = Config::from_yaml("./juno.yaml").unwrap();
    // log config contents
    println!("{}", config.chain_cfg.grpc_endpoint);

    let mut cosm_orc = CosmOrc::new(config, false).unwrap();
    let key = SigningKey {
        name: "validator".to_string(),
        key: Key::Mnemonic(std::env::var("MNEMONIC").unwrap()),
    };

    cosm_orc
        .contract_map
        .add_address(
            "nopixels",
            "juno1jde6srua84gamus3dgcfc2q20zrkxd46wex2f5nrv7wlj0sjcjys3663he",
        )
        .unwrap();

    let max_width = 32;
    let max_height = 32;
    let mut draft: image::RgbaImage = image::RgbaImage::new(max_width, max_height);
    let mut im = image::open(std::env::var("IMAGE").unwrap()).unwrap();
    let (width, height) = im.dimensions();
    // apply max width and max height to image
    if width > max_width {
        im = im.resize(max_width, height, image::imageops::FilterType::Nearest);
    }

    let chunk_x: u64 = std::env::var("SQUARE_X").unwrap().parse().unwrap();
    let chunk_y: u64 = std::env::var("SQUARE_Y").unwrap().parse().unwrap();
    let cooldown_seconds: u64 = std::env::var("COOLDOWN")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();

    // palette srgb color object from rgb: 255, 255, 255
    let white = palette::rgb::Srgb::from_components((255_u8, 255_u8, 255_u8));
    // rgb(228, 228, 228)
    let grey = palette::rgb::Srgb::from_components((228_u8, 228_u8, 228_u8));
    // rgb(136, 136, 136)
    let dark_grey = palette::rgb::Srgb::from_components((136_u8, 136_u8, 136_u8));
    // rgb(34, 34, 34)
    let black = palette::rgb::Srgb::from_components((34_u8, 34_u8, 34_u8));
    // rgb(240, 130, 125)
    let red = palette::rgb::Srgb::from_components((240_u8, 130_u8, 125_u8));
    // rgb(229, 0, 0)
    let dark_red = palette::rgb::Srgb::from_components((229_u8, 0_u8, 0_u8));
    // rgb(229, 149, 0)
    let orange = palette::rgb::Srgb::from_components((229_u8, 149_u8, 0_u8));
    // rgb(160, 106, 66)
    let brown = palette::rgb::Srgb::from_components((160_u8, 106_u8, 66_u8));
    // rgb(229, 217, 0)
    let yellow = palette::rgb::Srgb::from_components((229_u8, 217_u8, 0_u8));
    // rgb(148, 224, 68)
    let lime = palette::rgb::Srgb::from_components((148_u8, 224_u8, 68_u8));
    // rgb(2, 190, 1)
    let green = palette::rgb::Srgb::from_components((2_u8, 190_u8, 1_u8));
    // rgb(0, 211, 221)
    let cyan = palette::rgb::Srgb::from_components((0_u8, 211_u8, 221_u8));
    // rgb(0, 131, 199)
    let blue = palette::rgb::Srgb::from_components((0_u8, 131_u8, 199_u8));
    // rgb(0, 0, 234)
    let dark_blue = palette::rgb::Srgb::from_components((0_u8, 0_u8, 234_u8));
    // rgb(207, 110, 228)
    let light_purple = palette::rgb::Srgb::from_components((207_u8, 110_u8, 228_u8));
    // rgb(130, 0, 128)
    let purple = palette::rgb::Srgb::from_components((130_u8, 0_u8, 128_u8));

    let colors = vec![
        white,
        grey,
        dark_grey,
        black,
        red,
        dark_red,
        orange,
        brown,
        yellow,
        lime,
        green,
        cyan,
        blue,
        dark_blue,
        light_purple,
        purple,
    ];

    // function to calculate euclidean distance between two colors
    fn euclidean_distance(color1: &palette::rgb::Srgb, color2: &palette::rgb::Srgb) -> f32 {
        let (r1, g1, b1) = color1.into_components();
        let (r2, g2, b2) = color2.into_components();
        let r = r1 - r2;
        let g = g1 - g2;
        let b = b1 - b2;
        (r.powf(2_f32) + g.powf(2_f32) + b.powf(2_f32)).sqrt()
    }

    // function to find color using euclidean distance
    fn find_closest_color(
        color: &palette::rgb::Srgb<u8>,
        colors: &Vec<palette::rgb::Srgb<u8>>,
    ) -> (usize, palette::rgb::Srgb<u8>) {
        // search colors for closest match using euclidean distance of rgb values
        let mut closest_color = colors[0];
        let mut closest_distance =
            euclidean_distance(&closest_color.into_format(), &color.into_format());
        let mut closest_color_index = 0;
        for (i, c) in colors.iter().enumerate() {
            let dist = euclidean_distance(&color.into_format(), &c.into_format());
            if dist < closest_distance {
                closest_color = *c;
                closest_distance = dist;
                closest_color_index = i;
            }
        }
        (closest_color_index, closest_color)
    }

    let res = match cosm_orc.query(
        "nopixels",
        &juno_pixel::msg::QueryMsg::GetChunk {
            x: chunk_x,
            y: chunk_y,
        },
    ) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("loaded chunk");

    let mut did_draw = false;
    let mut has_drawn = false;
    let res: juno_pixel::msg::ChunkResponse = res.data().unwrap();
    // create iterable grid over max width and max height
    (0..max_width)
        .flat_map(move |x| (0..max_height).map(move |y| (x, y)))
        .for_each(|(x, y)| {
            // get pixel at x, y
            let pixel = im.get_pixel(x, y);
            // get pixel opacity
            let opacity = pixel.0[3];
            let (closest_color_index, closest_color) = if opacity > 0 {
                // convert pixel to srgb color
                let color = Srgb::from_components((pixel.0[0], pixel.0[1], pixel.0[2]));
                // find closest color in palette
                find_closest_color(&color, &colors)
            } else {
                // uncomment to clear out transparent space with white pixels. useful for
                // (0, white)
                return;
            };
            let closest_color_pixel = image::Rgba([
                closest_color.red,
                closest_color.green,
                closest_color.blue,
                1,
            ]);

            loop {
                if did_draw {
                    has_drawn = true;
                    // ensures two juno blocks have occured, enough time for the cooldown to complete
                    std::thread::sleep(std::time::Duration::from_secs(cooldown_seconds));
                } else if has_drawn {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
                did_draw = false;

                let pixel: juno_pixel::state::PixelInfo = res.grid[y as usize][x as usize].clone();
                // write closest color to pixel
                draft.put_pixel(x, y, closest_color_pixel);
                println!("{} vs {}", closest_color_index, pixel.color);
                if closest_color_index == pixel.color as usize {
                    println!("already correct color!");
                    return;
                };
                match cosm_orc.execute(
                    "nopixels",
                    "draw",
                    &juno_pixel::msg::ExecuteMsg::Draw {
                        chunk_x,
                        chunk_y,
                        x: x.into(),
                        y: y.into(),
                        color: closest_color_index.try_into().unwrap(),
                    },
                    &key,
                    vec![],
                ) {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => {
                        println!("{}", e);
                        println!("retrying...");
                    }
                }
            }
            did_draw = true;
            draft.save("draft.jpg").unwrap();

            println!(
                "drawn pixel {}, {} ({}/{})",
                x,
                y,
                (x + 1) * (y + 1),
                max_height * max_width
            );
        });
}
