use strum::Display;
use web_sys::{
    AnimationEvent, DragEvent, FocusEvent, InputEvent, KeyboardEvent, MouseEvent, PointerEvent,
    ProgressEvent, TouchEvent, TransitionEvent, WheelEvent,
};
use yew::{html, Callback};

use crate::callback_holder;

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Events {
    // Yew supported events
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
}

impl callback_holder::Callback for Events {
    type Event = web_sys::Event;

    fn get_event_type(&self) -> String {
        self.to_string()
    }

    fn get_callback(&self) -> &yew::Callback<web_sys::Event> {
        match self {
            Events::Abort(cb) => cb,
            Events::Cancel(cb) => cb,
            Events::CanPlay(cb) => cb,
            Events::CanPlayThrough(cb) => cb,
            Events::Change(cb) => cb,
            Events::Close(cb) => cb,
            Events::CueChange(cb) => cb,
            Events::DurationChange(cb) => cb,
            Events::Emptied(cb) => cb,
            Events::Ended(cb) => cb,
            Events::Error(cb) => cb,
            Events::FormData(cb) => cb,
            Events::Invalid(cb) => cb,
            Events::Load(cb) => cb,
            Events::LoadedData(cb) => cb,
            Events::LoadedMetadata(cb) => cb,
            Events::Pause(cb) => cb,
            Events::Play(cb) => cb,
            Events::Playing(cb) => cb,
            Events::RateChange(cb) => cb,
            Events::Reset(cb) => cb,
            Events::Resize(cb) => cb,
            Events::Scroll(cb) => cb,
            Events::SecurityPolicyViolation(cb) => cb,
            Events::Seeked(cb) => cb,
            Events::Seeking(cb) => cb,
            Events::Select(cb) => cb,
            Events::SlotChange(cb) => cb,
            Events::Stalled(cb) => cb,
            Events::Suspend(cb) => cb,
            Events::TimeUpdate(cb) => cb,
            Events::Toggle(cb) => cb,
            Events::VolumeChange(cb) => cb,
            Events::Waiting(cb) => cb,
            Events::Copy(cb) => cb,
            Events::Cut(cb) => cb,
            Events::Paste(cb) => cb,
            Events::SelectionChange(cb) => cb,
            Events::SelectStart(cb) => cb,
            Events::Show(cb) => cb,
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
    PointerLockChange(Callback<html::onpointerlockchange::Event>),
    PointerLockError(Callback<html::onpointerlockerror::Event>),
    PointerMove(Callback<html::onpointermove::Event>),
    PointerOut(Callback<html::onpointerout::Event>),
    PointerOver(Callback<html::onpointerover::Event>),
    PointerUp(Callback<html::onpointerup::Event>),
}

impl callback_holder::Callback for PointerEvents {
    type Event = PointerEvent;

    fn get_event_type(&self) -> String {
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
        todo!()
    }

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
        todo!()
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
