use a3::block::Block;

fn main() {
    let x = 1;
    let mut b0 = Block::initial(16);
    b0.mine(x);
    println!("{}", b0.hash_string());
    println!("{:02x}", b0.hash());
    let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
    b1.mine(x);
    println!("{}", b1.hash_string());
    println!("{:02x}", b1.hash());
    let mut b2 = Block::next(&b1, String::from("this is not interesting"));
    b2.mine(x);
    println!("{}", b2.hash_string());
    println!("{:02x}", b2.hash());

    
}

