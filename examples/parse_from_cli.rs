use metar::Metar;

fn main() -> anyhow::Result<()> {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0); // remove program call itself
    for arg in &args {
        let r = Metar::parse(arg)?;
        println!("'{arg}': {:#?}", r);
    }
    Ok(())
}
