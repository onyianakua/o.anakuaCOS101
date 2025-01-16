// declare a structure
struct Laptops{
    hp:u32, ibm:u32, toshiba:u32, dell:u32
}
// calculation of total price of laptop
impl Laptops{
    fn total(&self)->u32 {
        //use the . operator to fetch the value of a field via the self keyword
        3* (self.hp) + 3 *(self.ibm) + 3 *(self.toshiba) + 3 * (self.dell)


    }
}
fn main() {
    //represent the structure 
    let small = Laptops{
        hp:650000,
        ibm:755000,
        toshiba:550000,
        dell:850000,
    };
    // print the total price of Laptops
    println!(" hp = {} \n ibm = {} toshiba = {}\n dell = {} total of Laptops = {}",small.hp,small.ibm,small.toshiba,small.dell,small.total());
}