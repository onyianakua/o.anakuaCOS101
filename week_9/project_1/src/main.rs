use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {     //Define the drink categories
    let lager = vec!["33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
    let stout = vec!["Legend","Turbo King","Williams"];
    let non_alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];    //Display the categories


    println!("{:<15} {:<15} {:<15}","Lager", "Stout", "Non_alcoholic" );
    let max_len = lager.len().max(stout.len()).max(non_alcoholic.len());
    for i in 0..max_len {
        let lager_item = lager.get(i).unwrap_or(&"");
        let stout_item = stout.get(i).unwrap_or(&"");
        let non_alcoholic_item = non_alcoholic.get(i).unwrap_or(&"");
        println!("{:<15} {:<15} {:<15}", lager_item, stout_item, non_alcoholic_item);
    }
    // Save the data to a file
    let mut file = File::create("drinks_categories.csv")?;
    writeln!(file,"Lager,Stout,Non_alcoholic")?;
    for i in 0..max_len {
        let lager_item = lager.get(i).unwrap_or(&"");
        let stout_item = stout.get(i).unwrap_or(&"");
        let non_alcoholic_item = non_alcoholic.get(i).unwrap_or(&"");
        writeln!(file, "{},{},{}",lager_item,stout_item,non_alcoholic_item)?;
    }
    println!("Drink categories saved to 'drinks_categories.csv'");

    Ok(())

}


