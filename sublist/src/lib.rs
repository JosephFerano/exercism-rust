#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// Most answers use `windows()` but I figured it would be fun to do it manually
fn find_sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> bool {
    let mut ia = 0;
    let mut ib = 0;
    let mut start = 0;
    loop {
        if list1[ia] == list2[ib] {
            ia += 1;
            ib += 1;
            if ia >= list1.len() {
                return true
            } else if ib >= list2.len() {
                return false
            }
            continue
        } else if ia < list1.len() && ib < list2.len() {
            start += 1;
            ib = start;
            ia = 0;
            if ib >= list2.len() {
                return false
            }
            continue
        }
        return false
    }
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (_, 0) => Comparison::Superlist,
        (0, _) => Comparison::Sublist,
        (_, _) => {
            match (find_sublist(&first_list, &second_list), find_sublist(&second_list, &first_list)) {
                (true, true) => Comparison::Equal,
                (true, _) => Comparison::Sublist,
                (_, true) => Comparison::Superlist,
                (_,_)     => Comparison::Unequal
            }
        }
    }
}
