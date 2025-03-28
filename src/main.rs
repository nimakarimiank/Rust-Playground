fn main() {
    enum Coins {
        Penny, Nickel,Dime,Quarter
    }
    fn value_in_cents(coin: Coins) -> u8
    {
        match coin  {
            Coins::Penny=> 1,
            Coins::Nickel => 5,
            Coins::Dime=>10,
            Coins::Quarter=>25
        }
    }
// let array = [[4;4];3];
//     println!("{:#?}", array);


    let mut count:i32=0;
    'outerloop: loop{
        println!("count={}",count);
        let mut remaining = 10;
        loop{
            println!("remaining = {remaining}");
            if remaining == 5{
                break;
            }
            if count == 1{
                break 'outerloop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End!");
    let first_name = String::from("Nima");
    let middle_name = first_name.clone(); // deep copy
    println!("{first_name} {middle_name}");
    let first_name_shallow = first_name;//here first_name var gets invalidated
    println!("{first_name_shallow}");
    // println!("{first_name}");//throws an error
}