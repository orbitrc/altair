struct MyExtension;

#[no_mangle]
#[allow(non_snake_case)]
pub fn MyExtension__get_older(this: *mut MyExtension) {
    let now = MyExtension__age(this);
    MyExtension__set_age(this, now + 1);
}