use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use raystack_blocking::{new_client, ValueExt};

    let mut client = new_client("https://www.example.com/api/projName/", "username", "p4ssw0rd")?;

    let sites_grid = client.eval("readAll(site)")?;

    // Print the raw JSON:
    println!("{}", sites_grid.to_json_string_pretty());

    // Working with the Grid struct:
    println!("All columns: {:?}", sites_grid.cols());
    println!(
        "first site id: {:?}",
        sites_grid.rows()[0]["id"].as_hs_ref().unwrap()
    );

    Ok(())
}
