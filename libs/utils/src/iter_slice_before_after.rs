pub fn iter_slice_before_after<T, U>(slice: &mut [T], mut callback: U)
    where U : FnMut(&mut [T], &mut T, &mut [T]) {
    for i in 0..slice.len() {
        let (before, tmp) = slice.split_at_mut(i);
        let (item, after) = tmp.split_at_mut(1);
        callback(before, &mut item[0], after);
    }
}

#[cfg(test)] use test;

#[bench]
fn bench(b: &mut test::Bencher){
    let mut vec = [0usize;1000];
    for i in 0..1000 { vec[i] = i; }
    b.iter(||
        iter_slice_before_after(&mut vec, |before, item, after| {
            test::black_box(before);
            test::black_box(item);
            test::black_box(after);
        })
    );
}

#[test]
fn test_current(){
    let mut v = [1, 2, 3, 4];
    let clone = v.clone();
    let mut index = 0;
    let len = v.len();
    iter_slice_before_after(&mut v, |before, item, after| {
        assert!(before.len() == index);
        assert_eq!(*item, clone[index]);
        assert!(after.len() == len - index - 1);
        *item += 1;

        index += 1;
    });

    assert_eq!(v, [2, 3, 4, 5]);
}
    
#[test]
fn test_before(){
    let mut v = [1, 2, 3, 4];
    let clone = v.clone();
    let mut index = 0;
    let len = v.len();
    iter_slice_before_after(&mut v, |before, item, after| {
        assert!(before.len() == index);
        assert_eq!(*item, clone[index]);
        assert!(after.len() == len - index - 1);
        if before.len() > 0 { 
            let len = before.len() - 1;
            before[len] -= 1;
        }

        index += 1;
    });

    assert_eq!(v, [0, 1, 2, 4]);
}

#[test]
fn test_after(){
    let mut v = [1, 2, 3, 4];
    let clone = v.clone();
    let mut index = 0;
    let len = v.len();
    iter_slice_before_after(&mut v, |before, item, after| {
        assert!(before.len() == index);
        if index == 0 { assert_eq!(*item, clone[index]); }
        else { assert_eq!(*item, clone[index] - 1); }
        assert!(after.len() == len - index - 1);
        if after.len() > 0 { after[0] -= 1; }

        index += 1;
    });

    assert_eq!(v, [1, 1, 2, 3]);
}

#[test]
fn test_all(){
    let mut v = [1, 2, 3, 4];
    let mut index = 0;
    let len = v.len();
    iter_slice_before_after(&mut v, |before, item, after| {
        assert!(before.len() == index);
        if index == 0 { assert_eq!(*item, 1); }
        else { assert_eq!(*item, index + 2); }
        assert!(after.len() == len - index - 1);
        
        if before.len() > 0 { 
            let len = before.len() - 1;
            before[len] += 1;
        }
        *item += 1;
        if after.len() > 0 { after[0] += 1; }

        index += 1;
    });

    assert_eq!(v, [3, 5, 6, 6]);
}