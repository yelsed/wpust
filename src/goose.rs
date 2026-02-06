use color_eyre::eyre::Result;
use rascii_art::{
    render_to,
    RenderOptions,
};

pub fn goose(_site: String) -> Result<()> 
    let mut buffer = String::new();
                                                            
    render_to(
        r"/goose.png",
        &mut buffer,
        &RenderOptions::new()
            .width(100)
            .colored(true)
            .charset(&[".", ",", "-", "*", "£", "$", "#"]),
    )
    .unwrap()
    Ok(())
}
