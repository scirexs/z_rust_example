fn main() {
    use std::collections::HashMap;
    let int_ary = [11, 22, 33, 44, 55];
    let time_ary = ["2024-03-12T23:36:59Z", "2023-02-11T22:35:58Z", "2022-01-10T21:34:57Z"];
    let map = HashMap::from([(1, "foo"),(2, "bar")]);

    assert_eq!(time_ary.iter().flat_map(|&s| s.split(&['-',':','T'])).all(|s| s.parse::<i32>().is_ok()), false);
    assert_eq!(int_ary.iter().filter(|&&x| x % 2 != 0).find(|&&x| x == 77), None);
    assert_eq!(int_ary.iter().map(|&x| x + 1).inspect(|x| print!("{}", x)).fold(0, |acc, x| (acc / 2) + x), 90); //1223344556
    time_ary.iter().filter_map(|&s| s.find('4')).zip(int_ary).enumerate().for_each(|x| println!("{:?}", x));
    //(0, (3, 11))
    //(1, (15, 22))
    map.keys().filter_map(|k| map.get_key_value(k)).for_each(|(&x, &y)| println!("{}, {}", x, y));
    //1, foo
    //2, bar
}