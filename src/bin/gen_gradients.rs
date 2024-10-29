use anyhow::Result;
use ledplayr::patterns;

fn main() -> Result<()> {
    let pat = [
        patterns::Pattern::Spectral,
        patterns::Pattern::Blues,
        patterns::Pattern::Greens,
        patterns::Pattern::Greys,
        patterns::Pattern::Oranges,
        patterns::Pattern::Purples,
        patterns::Pattern::Reds,
        patterns::Pattern::Turbo,
        patterns::Pattern::Viridis,
        patterns::Pattern::Inferno,
        patterns::Pattern::Magma,
        patterns::Pattern::Plasma,
        patterns::Pattern::Cividis,
        patterns::Pattern::Warm,
        patterns::Pattern::Cool,
        patterns::Pattern::CubeHelix,
        patterns::Pattern::Sinebow,
        patterns::Pattern::Rainbow,
    ];

    for p in pat.iter() {
        let grad = p.as_gradient();
        let width = 250;

        let mut img = image::ImageBuffer::new(width, 10);
        for (x, _, pixel) in img.enumerate_pixels_mut() {
            let rgba = grad.at(x as f32 / width as f32).to_rgba8();
            *pixel = image::Rgba(rgba);
        }

        let name = serde_json::to_string(p);
        match name {
            Ok(name) => {
                let name = name.replace('"', "");
                let filename = format!("./web/static/gradients/{}.png", name);
                println!("Writing {}", filename);
                img.save(filename)?;
            }
            Err(e) => {
                println!("Could not convert pattern name: {:?} {}", p, e);
            }
        }
    }
    Ok(())
}
