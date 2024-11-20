use std::fmt::Debug;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, WindowEvent, MouseButton, KeyEvent, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::AHAppCmdBuffer;

pub(crate) type AHEventHandler<UserEvent>=Box<dyn Fn(AHEvents<UserEvent>)->AHAppCmdBuffer>;

#[macro_export]
macro_rules! is_event_requested {
    ($self:expr, $event:pat) => {
        // Return the result of iterating over the queue and matching the event
        if $self.queue.iter().any(|e| matches!(e, AHEvent::Window($event))){
            tracing::info!("requested event checked : {:?}",stringify!($event));

        }
    };
}


#[derive(Debug,Clone,Default)]
pub struct AHEvents<UserEvent:'static+Clone+Debug+Default>{
    // Window events
    close_requested:bool,
    resized:Option<PhysicalSize<u32>>,
    focused:bool,
    keyboard_events:Vec<KeyEvent>,

    cursor_moved:Option<PhysicalPosition<f64>>,
    mouse_events : Vec<( MouseButton, ElementState)>,
    mouse_wheel_delta:Option<MouseScrollDelta>,
    user_events:Vec<UserEvent>,

}
impl<UserEvent:'static+Clone+Debug+Default> AHEvents<UserEvent>{
    pub fn dispatch_window_event(&mut self,event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.close_requested=true,
            WindowEvent::Resized(size) => self.resized = Some(size),
            WindowEvent::Focused(focused) => self.focused = focused,
            WindowEvent::KeyboardInput { event, .. } => self.keyboard_events.push(event),
            WindowEvent::CursorMoved { position, .. } =>self.cursor_moved = Some(position),
            WindowEvent::MouseInput { state, button, .. } =>self.mouse_events.push((button, state)),
            WindowEvent::MouseWheel { delta, .. } => self.mouse_wheel_delta = Some(delta),
            _ => {},
        }
    }
    pub fn dispatch_user_event(&mut self,event: UserEvent) {
        self.user_events.push(event);
    }

    pub(crate) fn clear(&mut self) {
        self.resized = None;
        self.focused = false;
        self.keyboard_events.clear();
        self.cursor_moved = None;
        self.mouse_events.clear();
        self.mouse_wheel_delta = None;
        self.user_events.clear();
        self.close_requested=false;

    }
    //----------------------------------------------------------
    //  event handeling for user
    //----------------------------------------------------------
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
    pub fn resized(&self) -> Option<PhysicalSize<u32>>{
        self.resized
    }
    pub fn focused(&self) -> bool {
        self.focused
    }
    pub fn cursor_moved(&self) -> Option<PhysicalPosition<f64>> {
        self.cursor_moved
    }
    pub fn mouse_wheel_scrolled(&self) -> Option<MouseScrollDelta> {
        self.mouse_wheel_delta
    }
    pub fn key_pressed(&self,code:KeyCode)->bool{
        for event in &self.keyboard_events{
            if let KeyEvent{physical_key:PhysicalKey::Code(key_code),state:ElementState::Pressed,..}=event{
                if key_code.eq(&code) {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn key_released(&self,code:KeyCode)->bool{
        for event in &self.keyboard_events{
            if let KeyEvent{physical_key:PhysicalKey::Code(key_code),state:ElementState::Released,..}=event{
                if key_code.eq(&code) {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn key_held(&self,code:KeyCode)->bool{
        for event in &self.keyboard_events{
            if let KeyEvent{physical_key:PhysicalKey::Code(key_code),repeat:true,..}=event{
                if key_code.eq(&code) {
                    return true;
                }
            }
        }
        return false;
    }
}