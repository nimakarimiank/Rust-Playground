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
let array = [[4;4];3];
    println!("{:#?}", array);
}