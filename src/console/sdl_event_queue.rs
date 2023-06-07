use sdl2::event::Event;
use sdl2::EventPump;

pub struct SdlEventQueue<'a> {
  events: &'a mut EventPump,
  wait: bool,
}

impl<'a> SdlEventQueue<'a> {
  pub fn new(events: &'a mut EventPump) -> Self {
    SdlEventQueue { events, wait: true }
  }
}

impl<'a> Iterator for SdlEventQueue<'a> {
  type Item = Event;

  fn next(&mut self) -> Option<Event> {
    if self.wait {
      self.wait = false;
      Some(self.events.wait_event())
    } else {
      self.events.poll_event()
    }
  }
}
