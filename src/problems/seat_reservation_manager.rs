// 1845. 座位预约管理系统
// https://leetcode.cn/problems/seat-reservation-manager
struct SeatManager {
    seats: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        Self {
            seats: (1..=n).rev().collect(),
        }
    }

    fn reserve(&mut self) -> i32 {
        self.seats.pop().unwrap()
    }

    fn unreserve(&mut self, seat_number: i32) {
        let idx = match self.seats.binary_search_by(|s| seat_number.cmp(s)) {
            Ok(i) => i,
            Err(i) => i,
        };
        if idx == self.seats.len() {
            self.seats.push(seat_number);
        } else {
            self.seats.insert(idx, seat_number);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SeatManager;

    #[test]
    fn case_1() {
        let mut mgr = SeatManager::new(2);
        dbg!(mgr.reserve());
        mgr.unreserve(1);
        dbg!(mgr.reserve());
        dbg!(mgr.reserve());
        mgr.unreserve(2);
        dbg!(mgr.reserve());
        mgr.unreserve(1);
        dbg!(mgr.reserve());
        mgr.unreserve(2);
        dbg!(mgr.reserve());
    }
}
