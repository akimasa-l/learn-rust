mod hosting {
    fn add_to_list() {}
    fn seat_at_table() {
        super::super::func(); //相対パス
        super::f(); //相対パス
    }
}
pub fn f() {
        serving::take_order(); //相対パス
}
mod serving {
    pub fn take_order() {}
    fn serve_order() {}
    fn take_payment() {
        crate::front_of_house::serving::take_order() //絶対パス
    }
}