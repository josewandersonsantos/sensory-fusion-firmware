// ! Implement soon...

trait UsbClass
{
    fn handle_setup(&mut self, setup: &[u8]) -> bool;
    fn handle_out(&mut self, ep: u8, data: &[u8]);
    fn handle_in(&mut self, ep: u8);
}