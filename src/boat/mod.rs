pub(crate) mod motor;
pub(crate) mod rudder;

pub(crate) struct Boat<'a> {
    pub(crate) motor: motor::Motor<'a>,
    pub(crate) rudder: rudder::Rudder<'a>,
}
