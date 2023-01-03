use std::io;

fn main() {
    println!("Enter a date: (DD.MM.YY)");
    let mut date=String::new();
    io::stdin()
        .read_line(&mut date)
        .expect("");
    //extract day
    let day1= date.chars().nth(0).unwrap();
    let day2= date.chars().nth(1).unwrap();
    let day= format!("{}{}",day1, day2);
    let day= day.trim().parse::<usize>().expect("");

    //extract month
    let month1= date.chars().nth(3).unwrap();
    let month2= date.chars().nth(4).unwrap();
    let month= format!("{}{}",month1,month2);
    let month= month.trim().parse::<usize>().expect("");

    //extract year
    let year1= date.chars().nth(6).unwrap();
    let year2= date.chars().nth(7).unwrap();
    let year3= date.chars().nth(8).unwrap();
    let year4= date.chars().nth(9).unwrap();
    let year= format!("{}{}{}{}",year1,year2,year3,year4);
    let year= year.trim().parse::<usize>().expect("");


    //check date
    if ((month==2) && ((year%4==0) && (day>0 && day<=29)) || (year%100!=0) && (day>0 && day <=28)) ||
       (((month == 4 || month == 6 || month == 9 || month == 11) && (day > 0 && day <= 30)) ||
       ((month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12) &&
       (day > 0 && day <= 31))) {
       let date= format!("{day}.{month}.{year}");
        println!("{date}");
    }else{
        println!("Date does not exist!");
    }
}
