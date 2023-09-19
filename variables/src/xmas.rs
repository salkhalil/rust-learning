pub fn twelve_days() {
    const DAYS: usize = 12;

    for day in 0..DAYS {
        write_verse(day);
        println!("\n");
    }
}

fn write_verse(num: usize) {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelth",
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    println!(
        "On the {} day of Christmas, my true love sent to me",
        days[num]
    );
    for day in (0..num + 1).rev() {
        println!("{}", gifts[day]);
    }
}

