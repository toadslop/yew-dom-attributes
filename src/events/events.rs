use crate::callback_holder;
use strum::Display;
use web_sys::{
    AnimationEvent, DragEvent, FocusEvent, InputEvent, KeyboardEvent, MouseEvent, PointerEvent,
    ProgressEvent, TouchEvent, TransitionEvent, WheelEvent,
};
use yew::{html, Callback};

#[derive(Debug, PartialEq, Clone)]
pub enum EventType {
    MouseEvent(MouseEvents),
    Event(GenericEvents),
    TouchEvent(TouchEvents),
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum GenericEvents {
    Abort(Callback<html::onabort::Event>),
    Cancel(Callback<html::oncancel::Event>),
    CanPlay(Callback<html::oncanplay::Event>),
    CanPlayThrough(Callback<html::oncanplaythrough::Event>),
    Change(Callback<html::onchange::Event>),
    Close(Callback<html::onclose::Event>),
    CueChange(Callback<html::oncuechange::Event>),
    DurationChange(Callback<html::ondurationchange::Event>),
    Emptied(Callback<html::onemptied::Event>),
    Ended(Callback<html::onended::Event>),
    Error(Callback<html::onerror::Event>),
    FormData(Callback<html::onformdata::Event>),
    Invalid(Callback<html::oninvalid::Event>),
    Load(Callback<html::onload::Event>),
    LoadedData(Callback<html::onloadeddata::Event>),
    LoadedMetadata(Callback<html::onloadedmetadata::Event>),
    Pause(Callback<html::onpause::Event>),
    Play(Callback<html::onplay::Event>),
    Playing(Callback<html::onplaying::Event>),
    RateChange(Callback<html::onratechange::Event>),
    Reset(Callback<html::onreset::Event>),
    Resize(Callback<html::onresize::Event>),
    Scroll(Callback<html::onscroll::Event>),
    SecurityPolicyViolation(Callback<html::onsecuritypolicyviolation::Event>),
    Seeked(Callback<html::onseeked::Event>),
    Seeking(Callback<html::onseeking::Event>),
    Select(Callback<html::onselect::Event>),
    SlotChange(Callback<html::onslotchange::Event>),
    Stalled(Callback<html::onstalled::Event>),
    Suspend(Callback<html::onsuspend::Event>),
    TimeUpdate(Callback<html::ontimeupdate::Event>),
    Toggle(Callback<html::ontoggle::Event>),
    VolumeChange(Callback<html::onvolumechange::Event>),
    Waiting(Callback<html::onwaiting::Event>),
    Copy(Callback<html::oncopy::Event>),
    Cut(Callback<html::oncut::Event>),
    Paste(Callback<html::onpaste::Event>),
    SelectionChange(Callback<html::onselectionchange::Event>),
    SelectStart(Callback<html::onselectstart::Event>),
    Show(Callback<html::onshow::Event>),
    PointerLockChange(Callback<html::onpointerlockchange::Event>),
    PointerLockError(Callback<html::onpointerlockerror::Event>),
}

impl callback_holder::Callback for GenericEvents {
    type Event = web_sys::Event;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<web_sys::Event> {
        match self {
            GenericEvents::Abort(cb) => cb,
            GenericEvents::Cancel(cb) => cb,
            GenericEvents::CanPlay(cb) => cb,
            GenericEvents::CanPlayThrough(cb) => cb,
            GenericEvents::Change(cb) => cb,
            GenericEvents::Close(cb) => cb,
            GenericEvents::CueChange(cb) => cb,
            GenericEvents::DurationChange(cb) => cb,
            GenericEvents::Emptied(cb) => cb,
            GenericEvents::Ended(cb) => cb,
            GenericEvents::Error(cb) => cb,
            GenericEvents::FormData(cb) => cb,
            GenericEvents::Invalid(cb) => cb,
            GenericEvents::Load(cb) => cb,
            GenericEvents::LoadedData(cb) => cb,
            GenericEvents::LoadedMetadata(cb) => cb,
            GenericEvents::Pause(cb) => cb,
            GenericEvents::Play(cb) => cb,
            GenericEvents::Playing(cb) => cb,
            GenericEvents::RateChange(cb) => cb,
            GenericEvents::Reset(cb) => cb,
            GenericEvents::Resize(cb) => cb,
            GenericEvents::Scroll(cb) => cb,
            GenericEvents::SecurityPolicyViolation(cb) => cb,
            GenericEvents::Seeked(cb) => cb,
            GenericEvents::Seeking(cb) => cb,
            GenericEvents::Select(cb) => cb,
            GenericEvents::SlotChange(cb) => cb,
            GenericEvents::Stalled(cb) => cb,
            GenericEvents::Suspend(cb) => cb,
            GenericEvents::TimeUpdate(cb) => cb,
            GenericEvents::Toggle(cb) => cb,
            GenericEvents::VolumeChange(cb) => cb,
            GenericEvents::Waiting(cb) => cb,
            GenericEvents::Copy(cb) => cb,
            GenericEvents::Cut(cb) => cb,
            GenericEvents::Paste(cb) => cb,
            GenericEvents::SelectionChange(cb) => cb,
            GenericEvents::SelectStart(cb) => cb,
            GenericEvents::Show(cb) => cb,
            GenericEvents::PointerLockChange(cb) => cb,
            GenericEvents::PointerLockError(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum MouseEvents {
    AuxClick(Callback<html::onauxclick::Event>),
    Click(Callback<html::onclick::Event>),
    ContextMenu(Callback<html::oncontextmenu::Event>),
    DblClick(Callback<html::ondblclick::Event>),
    MouseDown(Callback<html::onmousedown::Event>),
    MousEenter(Callback<html::onmouseenter::Event>),
    MouseLeave(Callback<html::onmouseleave::Event>),
    MouseMove(Callback<html::onmousemove::Event>),
    MouseOut(Callback<html::onmouseout::Event>),
    MouseOver(Callback<html::onmouseover::Event>),
    MouseUp(Callback<html::onmouseup::Event>),
}

impl callback_holder::Callback for MouseEvents {
    type Event = MouseEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<MouseEvent> {
        match self {
            MouseEvents::AuxClick(cb) => cb,
            MouseEvents::Click(cb) => cb,
            MouseEvents::ContextMenu(cb) => cb,
            MouseEvents::DblClick(cb) => cb,
            MouseEvents::MouseDown(cb) => cb,
            MouseEvents::MousEenter(cb) => cb,
            MouseEvents::MouseLeave(cb) => cb,
            MouseEvents::MouseMove(cb) => cb,
            MouseEvents::MouseOut(cb) => cb,
            MouseEvents::MouseOver(cb) => cb,
            MouseEvents::MouseUp(cb) => cb,
        }
    }
}
#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum FocusEvents {
    Blur(Callback<html::onblur::Event>),
    Focus(Callback<html::onfocus::Event>),
    FocusIn(Callback<html::onfocusin::Event>),
    FocusOut(Callback<html::onfocusout::Event>),
    Submit(Callback<html::onsubmit::Event>),
}

impl callback_holder::Callback for FocusEvents {
    type Event = FocusEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            FocusEvents::Blur(cb) => cb,
            FocusEvents::Focus(cb) => cb,
            FocusEvents::FocusIn(cb) => cb,
            FocusEvents::FocusOut(cb) => cb,
            FocusEvents::Submit(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum DragEvents {
    Drag(Callback<html::ondrag::Event>),
    DragEnd(Callback<html::ondragend::Event>),
    DragEnter(Callback<html::ondragenter::Event>),
    DragExit(Callback<html::ondragexit::Event>),
    DragLeave(Callback<html::ondragleave::Event>),
    DragOver(Callback<html::ondragover::Event>),
    DragStart(Callback<html::ondragstart::Event>),
    Drop(Callback<html::ondrop::Event>),
}

impl callback_holder::Callback for DragEvents {
    type Event = DragEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            DragEvents::Drag(cb) => cb,
            DragEvents::DragEnd(cb) => cb,
            DragEvents::DragEnter(cb) => cb,
            DragEvents::DragExit(cb) => cb,
            DragEvents::DragLeave(cb) => cb,
            DragEvents::DragOver(cb) => cb,
            DragEvents::DragStart(cb) => cb,
            DragEvents::Drop(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum InputEvents {
    Input(Callback<html::oninput::Event>),
}

impl callback_holder::Callback for InputEvents {
    type Event = InputEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            InputEvents::Input(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum KeyboardEvents {
    Keydown(Callback<html::onkeydown::Event>),
    KeyPress(Callback<html::onkeypress::Event>),
    KeyUp(Callback<html::onkeyup::Event>),
}

impl callback_holder::Callback for KeyboardEvents {
    type Event = KeyboardEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            KeyboardEvents::Keydown(cb) => cb,
            KeyboardEvents::KeyPress(cb) => cb,
            KeyboardEvents::KeyUp(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum ProgressEvents {
    LoadStart(Callback<html::onloadstart::Event>),
    Progress(Callback<html::onprogress::Event>),
    Loadend(Callback<html::onloadend::Event>),
}

impl callback_holder::Callback for ProgressEvents {
    type Event = ProgressEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            ProgressEvents::LoadStart(cb) => cb,
            ProgressEvents::Progress(cb) => cb,
            ProgressEvents::Loadend(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum WheelEvents {
    Wheel(Callback<html::onwheel::Event>),
}

impl callback_holder::Callback for WheelEvents {
    type Event = WheelEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            WheelEvents::Wheel(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum AnimationEvents {
    AnimationCancel(Callback<html::onanimationcancel::Event>),
    AnimationEnd(Callback<html::onanimationend::Event>),
    AnimationIteration(Callback<html::onanimationiteration::Event>),
    AnimationStart(Callback<html::onanimationstart::Event>),
}

impl callback_holder::Callback for AnimationEvents {
    type Event = AnimationEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            AnimationEvents::AnimationCancel(cb) => cb,
            AnimationEvents::AnimationEnd(cb) => cb,
            AnimationEvents::AnimationIteration(cb) => cb,
            AnimationEvents::AnimationStart(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum PointerEvents {
    GotPointerCapture(Callback<html::ongotpointercapture::Event>),
    LostPointerCapture(Callback<html::onlostpointercapture::Event>),
    PointerCancel(Callback<html::onpointercancel::Event>),
    PointerDown(Callback<html::onpointerdown::Event>),
    PointerEnter(Callback<html::onpointerenter::Event>),
    PointerLeave(Callback<html::onpointerleave::Event>),
    PointerMove(Callback<html::onpointermove::Event>),
    PointerOut(Callback<html::onpointerout::Event>),
    PointerOver(Callback<html::onpointerover::Event>),
    PointerUp(Callback<html::onpointerup::Event>),
}

impl callback_holder::Callback for PointerEvents {
    type Event = PointerEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            PointerEvents::GotPointerCapture(cb) => cb,
            PointerEvents::LostPointerCapture(cb) => cb,
            PointerEvents::PointerCancel(cb) => cb,
            PointerEvents::PointerDown(cb) => cb,
            PointerEvents::PointerEnter(cb) => cb,
            PointerEvents::PointerLeave(cb) => cb,
            PointerEvents::PointerMove(cb) => cb,
            PointerEvents::PointerOut(cb) => cb,
            PointerEvents::PointerOver(cb) => cb,
            PointerEvents::PointerUp(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TouchEvents {
    TouchCancel(Callback<html::ontouchcancel::Event>),
    TouchEnd(Callback<html::ontouchend::Event>),
    TouchMove(Callback<html::ontouchmove::Event>),
    TouchStart(Callback<html::ontouchstart::Event>),
}

impl callback_holder::Callback for TouchEvents {
    type Event = TouchEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            TouchEvents::TouchCancel(cb) => cb,
            TouchEvents::TouchEnd(cb) => cb,
            TouchEvents::TouchMove(cb) => cb,
            TouchEvents::TouchStart(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TransitionEvents {
    TransitionCancel(Callback<html::ontransitioncancel::Event>),
    TransitionEnd(Callback<html::ontransitionend::Event>),
    TransitionRun(Callback<html::ontransitionrun::Event>),
    TransitionStart(Callback<html::ontransitionstart::Event>),
}

impl callback_holder::Callback for TransitionEvents {
    type Event = TransitionEvent;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        match self {
            TransitionEvents::TransitionCancel(cb) => cb,
            TransitionEvents::TransitionEnd(cb) => cb,
            TransitionEvents::TransitionRun(cb) => cb,
            TransitionEvents::TransitionStart(cb) => cb,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CustomEvent {
    event_type: String,
    callback: yew::Callback<web_sys::Event>,
}

impl CustomEvent {
    pub fn new(event_type: String, callback: yew::Callback<web_sys::Event>) -> Self {
        Self {
            event_type,
            callback,
        }
    }
}

impl callback_holder::Callback for CustomEvent {
    type Event = web_sys::Event;

    fn get_event_type(&self) -> String {
        self.event_type.to_owned()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        &self.callback
    }
}

pub trait EventPropsReceiver {
    fn add_generic_listener(&mut self, callback: GenericEvents);
    fn add_mouse_event_listener(&mut self, callback: MouseEvents);
    fn add_custom_listener(&mut self, callback: CustomEvent);
    fn add_transition_event_listener(&mut self, callback: TransitionEvents);
    fn add_touch_event_listener(&mut self, callback: TouchEvents);
    fn add_animation_event_listener(&mut self, callback: AnimationEvents);
    fn add_pointer_event_listener(&mut self, callback: PointerEvents);
    fn add_wheel_event_listener(&mut self, callback: WheelEvents);
    fn add_progress_event_listener(&mut self, callback: ProgressEvents);
    fn add_keyboard_event_listener(&mut self, callback: KeyboardEvents);
    fn add_drag_event_listener(&mut self, callback: DragEvents);
    fn add_input_event_listener(&mut self, callback: InputEvents);
    fn add_focus_event_listeners(&mut self, callback: FocusEvents);
}
