mod front_of_house;
mod back_of_house;

fn main(){
    println!("main");
    eat_at_restaurant();
}

pub fn eat_at_restaurant() {
    use front_of_house::{hosting, hosting::inner_hosting};
    hosting::add_to_waitlist();
    inner_hosting::inner_serve();
}
