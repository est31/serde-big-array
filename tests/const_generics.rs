use serde_big_array::BigArray;
use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(with = "BigArray")]
    arr: [u8; 64],
    #[serde(with = "BigArray")]
    arr2: [u8; 65],
}

#[test]
fn test() {
    let s = S {
        arr: [1; 64],
        arr2: [1; 65],
    };
    let j = serde_json::to_string(&s).unwrap();
    let s_back = serde_json::from_str::<S>(&j).unwrap();
    assert!(&s.arr[..] == &s_back.arr[..]);
}

// TODO add a test that drop is executed nicely if there is an error
// during deserialization

#[test]
fn test_droppped_partial() {
    thread_local! {
        static DROPPED: RefCell<Vec<u32>> = RefCell::new(Vec::new());
    }

    fn get_droppped_set() -> HashSet<u32> {
        DROPPED.with(|dropped| {
            dropped.borrow().iter().copied().collect::<HashSet<_>>()
        })
    }

    fn clear_droppped_set() {
        DROPPED.with(|dropped| {
            dropped.borrow_mut().clear()
        });
        assert_eq!(get_droppped_set().len(), 0);
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
    struct DroppableU32(u32);

    impl Drop for DroppableU32 {
        fn drop(&mut self) {
            DROPPED.with(|dropped| dropped.borrow_mut().push(self.0));
        }
    }

    const CNT :usize = 4;

    #[derive(Serialize, Deserialize, Debug)]
    struct Droppables {
        #[serde(with = "BigArray")]
        arr: [DroppableU32; CNT],
    }

    let mut maybe_init_array = core::mem::MaybeUninit::<[DroppableU32; CNT]>::uninit();
    for i in 0..CNT {
        unsafe {
            let p = (maybe_init_array.as_mut_ptr() as *mut DroppableU32).wrapping_add(i);
            core::ptr::write(p, DroppableU32(i as u32 * 3));
        }
    }

    let mut ds = Droppables {
        arr: unsafe { maybe_init_array.assume_init() },
    };

    const VAL_IDX: usize = 2;
    const VAL: u32 = 20220325;

    assert_eq!(get_droppped_set().len(), 0);
    ds.arr[VAL_IDX] = DroppableU32(VAL);
    assert_eq!(get_droppped_set(), vec![VAL_IDX as u32 * 3].into_iter().collect::<HashSet<_>>());
    clear_droppped_set();

    let j = serde_json::to_string(&ds).unwrap();
    println!("{}", j);

    let val_starts = j.find(&VAL.to_string()).unwrap();
    {
        let ds_back = serde_json::from_str::<Droppables>(&j).unwrap();
        assert!(&ds.arr[..] == &ds_back.arr[..]);
    }
    let mut zero_to_cnt_set :HashSet<u32> = (0..CNT as u32).map(|v| v * 3).into_iter().collect();

    zero_to_cnt_set.remove(&(VAL_IDX as u32 * 3));
    zero_to_cnt_set.insert(VAL);

    assert_eq!(get_droppped_set(), zero_to_cnt_set);
    clear_droppped_set();
    let _ds_back_err = serde_json::from_str::<Droppables>(&j[0..val_starts]).unwrap_err();

    let zero_to_val_idx_set :HashSet<u32> = (0..VAL_IDX as u32).map(|v| v * 3).into_iter().collect();

    assert_eq!(get_droppped_set(), zero_to_val_idx_set);
}
