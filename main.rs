fn main()
{
    // println!("Hello world");
    // let x: i64 = 4200000000000000;
    // let pi: f32 = 3.14;
    // let isSnowing: bool = true;
    // let one: char = 'a';

    // println!("hey: {}", pi);
    // println!("Sighned Integer: {}", x);
    // println!("is Snowing: {}" , isSnowing);
    // println!("hey: {}", one);

    //Array, tuples, Slices, and Strings (slice String)

    //Arrays

    let number: [f32; 5] = [1 as f32, 2 as f32, 2 as f32, 2 as f32, 2 as f32];
    let mix: [&str; 3] = ["Apple", "Banna", "hey"];
    let cool: (&str, i32, bool) = ("Hey", 30, false);

    //slices contagoious pieces of memor

    println!("Number array: {:?}", number);
    println!("Number array: {:?}", mix);
    println!("Number array: {}", mix[0]);
    println!("Number array: {:?}", cool);
}