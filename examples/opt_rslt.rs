fn main() {
    let all_some: [Option<i32>; 3] = [Some(1), Some(2), Some(3)];
    let part_none: [Option<i32>; 3] = [None, Some(1), None];
    let all_ok: [Result<i32, i32>; 4] = [Ok(1), Ok(2), Ok(3), Ok(4)];
    let part_err: [Result<i32, i32>; 4] = [Ok(1), Err(-2), Ok(3), Err(-4)];

    // 処理結果群が全て成功したかどうか判別できる
    assert_eq!(all_some.iter().copied().collect::<Option<Vec<_>>>(), Some(vec![1, 2, 3]));
    assert_eq!(part_none.iter().copied().collect::<Option<Vec<_>>>(), None);
    assert_eq!(all_ok.iter().copied().collect::<Result<Vec<_>,_>>(), Ok(vec![1, 2, 3, 4]));
    assert_eq!(part_err.iter().copied().collect::<Result<Vec<_>,_>>(), Err(-2));

    // 処理結果群の成功だけを抽出できる
    assert_eq!(all_some.iter().filter_map(|&x| x).collect::<Vec<_>>(), vec![1, 2, 3]);  //flat_mapでも可
    assert_eq!(part_none.iter().filter_map(|&x| x).collect::<Vec<_>>(), vec![1]);       //flat_mapでも可
    assert_eq!(all_ok.iter().flat_map(|&x| x).collect::<Vec<_>>(), vec![1, 2, 3, 4]);   //filter_map(|&x| x.ok())でも可
    assert_eq!(part_err.iter().flat_map(|&x| x).collect::<Vec<_>>(), vec![1, 3]);       //filter_map(|&x| x.ok())でも可
}