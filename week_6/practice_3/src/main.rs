fn main() {
    let name1 = "Ayomide Adesokan";
    println!("My name is {}", name1);

    let name2 = name1.replace("Ayomide", "Adebare");
    println!("you can also call me {}", name2);
    let faculty = "Falculty of Science and Technology";

    let school = faculty.replace("Falculty", "School");
    println!("I as a student of the {}", school);
}
