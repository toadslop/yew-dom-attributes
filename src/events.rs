use nanoid::nanoid;
use strum::Display;
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

    fn get_callback(&self) -> &yew::Callback<Self::Event> {
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

    fn get_id(&self) -> String {
        nanoid!()
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

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum FocusEvents {
    Blur(Callback<html::onblur::Event>),
    Focus(Callback<html::onfocus::Event>),
    FocusIn(Callback<html::onfocusin::Event>),
    FocusOut(Callback<html::onfocusout::Event>),
    Submit(Callback<html::onsubmit::Event>),
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

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum InputEvents {
    Input(Callback<html::oninput::Event>),
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum KeyboardEvents {
    Keydown(Callback<html::onkeydown::Event>),
    KeyPress(Callback<html::onkeypress::Event>),
    KeyUp(Callback<html::onkeyup::Event>),
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum ProgressEvents {
    LoadStart(Callback<html::onloadstart::Event>),
    Progress(Callback<html::onprogress::Event>),
    Loadend(Callback<html::onloadend::Event>),
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum WheelEvents {
    Wheel(Callback<html::onwheel::Event>),
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum AnimationEvents {
    AnimationCancel(Callback<html::onanimationcancel::Event>),
    AnimationEnd(Callback<html::onanimationend::Event>),
    AnimationIteration(Callback<html::onanimationiteration::Event>),
    AnimationStart(Callback<html::onanimationstart::Event>),
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

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TouchEvents {
    TouchCancel(Callback<html::ontouchcancel::Event>),
    TouchEnd(Callback<html::ontouchend::Event>),
    TouchMove(Callback<html::ontouchmove::Event>),
    TouchStart(Callback<html::ontouchstart::Event>),
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TransitionEvents {
    TransitionCancel(Callback<html::ontransitioncancel::Event>),
    TransitionEnd(Callback<html::ontransitionend::Event>),
    TransitionRun(Callback<html::ontransitionrun::Event>),
    TransitionStart(Callback<html::ontransitionstart::Event>),
}

// interface DOMAttributes<T> {
//     children?: ReactNode | undefined;
//     dangerouslySetInnerHTML?: {
//         __html: string;
//     } | undefined;

//     // Clipboard Events
//     onCopy?: ClipboardEventHandler<T> | undefined;
//     onCopyCapture?: ClipboardEventHandler<T> | undefined;
//     onCut?: ClipboardEventHandler<T> | undefined;
//     onCutCapture?: ClipboardEventHandler<T> | undefined;
//     onPaste?: ClipboardEventHandler<T> | undefined;
//     onPasteCapture?: ClipboardEventHandler<T> | undefined;

//     // Composition Events
//     onCompositionEnd?: CompositionEventHandler<T> | undefined;
//     onCompositionEndCapture?: CompositionEventHandler<T> | undefined;
//     onCompositionStart?: CompositionEventHandler<T> | undefined;
//     onCompositionStartCapture?: CompositionEventHandler<T> | undefined;
//     onCompositionUpdate?: CompositionEventHandler<T> | undefined;
//     onCompositionUpdateCapture?: CompositionEventHandler<T> | undefined;

//     // Focus Events
//     onFocus?: FocusEventHandler<T> | undefined;
//     onFocusCapture?: FocusEventHandler<T> | undefined;
//     onBlur?: FocusEventHandler<T> | undefined;
//     onBlurCapture?: FocusEventHandler<T> | undefined;

//     // Form Events
//     onChange?: FormEventHandler<T> | undefined;
//     onChangeCapture?: FormEventHandler<T> | undefined;
//     onBeforeInput?: FormEventHandler<T> | undefined;
//     onBeforeInputCapture?: FormEventHandler<T> | undefined;
//     onInput?: FormEventHandler<T> | undefined;
//     onInputCapture?: FormEventHandler<T> | undefined;
//     onReset?: FormEventHandler<T> | undefined;
//     onResetCapture?: FormEventHandler<T> | undefined;
//     onSubmit?: FormEventHandler<T> | undefined;
//     onSubmitCapture?: FormEventHandler<T> | undefined;
//     onInvalid?: FormEventHandler<T> | undefined;
//     onInvalidCapture?: FormEventHandler<T> | undefined;

//     // Image Events
//     onLoad?: ReactEventHandler<T> | undefined;
//     onLoadCapture?: ReactEventHandler<T> | undefined;
//     onError?: ReactEventHandler<T> | undefined; // also a Media Event
//     onErrorCapture?: ReactEventHandler<T> | undefined; // also a Media Event

//     // Keyboard Events
//     onKeyDown?: KeyboardEventHandler<T> | undefined;
//     onKeyDownCapture?: KeyboardEventHandler<T> | undefined;
//     /** @deprecated */
//     onKeyPress?: KeyboardEventHandler<T> | undefined;
//     /** @deprecated */
//     onKeyPressCapture?: KeyboardEventHandler<T> | undefined;
//     onKeyUp?: KeyboardEventHandler<T> | undefined;
//     onKeyUpCapture?: KeyboardEventHandler<T> | undefined;

//     // Media Events
//     onAbort?: ReactEventHandler<T> | undefined;
//     onAbortCapture?: ReactEventHandler<T> | undefined;
//     onCanPlay?: ReactEventHandler<T> | undefined;
//     onCanPlayCapture?: ReactEventHandler<T> | undefined;
//     onCanPlayThrough?: ReactEventHandler<T> | undefined;
//     onCanPlayThroughCapture?: ReactEventHandler<T> | undefined;
//     onDurationChange?: ReactEventHandler<T> | undefined;
//     onDurationChangeCapture?: ReactEventHandler<T> | undefined;
//     onEmptied?: ReactEventHandler<T> | undefined;
//     onEmptiedCapture?: ReactEventHandler<T> | undefined;
//     onEncrypted?: ReactEventHandler<T> | undefined;
//     onEncryptedCapture?: ReactEventHandler<T> | undefined;
//     onEnded?: ReactEventHandler<T> | undefined;
//     onEndedCapture?: ReactEventHandler<T> | undefined;
//     onLoadedData?: ReactEventHandler<T> | undefined;
//     onLoadedDataCapture?: ReactEventHandler<T> | undefined;
//     onLoadedMetadata?: ReactEventHandler<T> | undefined;
//     onLoadedMetadataCapture?: ReactEventHandler<T> | undefined;
//     onLoadStart?: ReactEventHandler<T> | undefined;
//     onLoadStartCapture?: ReactEventHandler<T> | undefined;
//     onPause?: ReactEventHandler<T> | undefined;
//     onPauseCapture?: ReactEventHandler<T> | undefined;
//     onPlay?: ReactEventHandler<T> | undefined;
//     onPlayCapture?: ReactEventHandler<T> | undefined;
//     onPlaying?: ReactEventHandler<T> | undefined;
//     onPlayingCapture?: ReactEventHandler<T> | undefined;
//     onProgress?: ReactEventHandler<T> | undefined;
//     onProgressCapture?: ReactEventHandler<T> | undefined;
//     onRateChange?: ReactEventHandler<T> | undefined;
//     onRateChangeCapture?: ReactEventHandler<T> | undefined;
//     onSeeked?: ReactEventHandler<T> | undefined;
//     onSeekedCapture?: ReactEventHandler<T> | undefined;
//     onSeeking?: ReactEventHandler<T> | undefined;
//     onSeekingCapture?: ReactEventHandler<T> | undefined;
//     onStalled?: ReactEventHandler<T> | undefined;
//     onStalledCapture?: ReactEventHandler<T> | undefined;
//     onSuspend?: ReactEventHandler<T> | undefined;
//     onSuspendCapture?: ReactEventHandler<T> | undefined;
//     onTimeUpdate?: ReactEventHandler<T> | undefined;
//     onTimeUpdateCapture?: ReactEventHandler<T> | undefined;
//     onVolumeChange?: ReactEventHandler<T> | undefined;
//     onVolumeChangeCapture?: ReactEventHandler<T> | undefined;
//     onWaiting?: ReactEventHandler<T> | undefined;
//     onWaitingCapture?: ReactEventHandler<T> | undefined;

//     // MouseEvents
//     onAuxClick?: MouseEventHandler<T> | undefined;
//     onAuxClickCapture?: MouseEventHandler<T> | undefined;
//     onClick?: MouseEventHandler<T> | undefined;
//     onClickCapture?: MouseEventHandler<T> | undefined;
//     onContextMenu?: MouseEventHandler<T> | undefined;
//     onContextMenuCapture?: MouseEventHandler<T> | undefined;
//     onDoubleClick?: MouseEventHandler<T> | undefined;
//     onDoubleClickCapture?: MouseEventHandler<T> | undefined;
//     onDrag?: DragEventHandler<T> | undefined;
//     onDragCapture?: DragEventHandler<T> | undefined;
//     onDragEnd?: DragEventHandler<T> | undefined;
//     onDragEndCapture?: DragEventHandler<T> | undefined;
//     onDragEnter?: DragEventHandler<T> | undefined;
//     onDragEnterCapture?: DragEventHandler<T> | undefined;
//     onDragExit?: DragEventHandler<T> | undefined;
//     onDragExitCapture?: DragEventHandler<T> | undefined;
//     onDragLeave?: DragEventHandler<T> | undefined;
//     onDragLeaveCapture?: DragEventHandler<T> | undefined;
//     onDragOver?: DragEventHandler<T> | undefined;
//     onDragOverCapture?: DragEventHandler<T> | undefined;
//     onDragStart?: DragEventHandler<T> | undefined;
//     onDragStartCapture?: DragEventHandler<T> | undefined;
//     onDrop?: DragEventHandler<T> | undefined;
//     onDropCapture?: DragEventHandler<T> | undefined;
//     onMouseDown?: MouseEventHandler<T> | undefined;
//     onMouseDownCapture?: MouseEventHandler<T> | undefined;
//     onMouseEnter?: MouseEventHandler<T> | undefined;
//     onMouseLeave?: MouseEventHandler<T> | undefined;
//     onMouseMove?: MouseEventHandler<T> | undefined;
//     onMouseMoveCapture?: MouseEventHandler<T> | undefined;
//     onMouseOut?: MouseEventHandler<T> | undefined;
//     onMouseOutCapture?: MouseEventHandler<T> | undefined;
//     onMouseOver?: MouseEventHandler<T> | undefined;
//     onMouseOverCapture?: MouseEventHandler<T> | undefined;
//     onMouseUp?: MouseEventHandler<T> | undefined;
//     onMouseUpCapture?: MouseEventHandler<T> | undefined;

//     // Selection Events
//     onSelect?: ReactEventHandler<T> | undefined;
//     onSelectCapture?: ReactEventHandler<T> | undefined;

//     // Touch Events
//     onTouchCancel?: TouchEventHandler<T> | undefined;
//     onTouchCancelCapture?: TouchEventHandler<T> | undefined;
//     onTouchEnd?: TouchEventHandler<T> | undefined;
//     onTouchEndCapture?: TouchEventHandler<T> | undefined;
//     onTouchMove?: TouchEventHandler<T> | undefined;
//     onTouchMoveCapture?: TouchEventHandler<T> | undefined;
//     onTouchStart?: TouchEventHandler<T> | undefined;
//     onTouchStartCapture?: TouchEventHandler<T> | undefined;

//     // Pointer Events
//     onPointerDown?: PointerEventHandler<T> | undefined;
//     onPointerDownCapture?: PointerEventHandler<T> | undefined;
//     onPointerMove?: PointerEventHandler<T> | undefined;
//     onPointerMoveCapture?: PointerEventHandler<T> | undefined;
//     onPointerUp?: PointerEventHandler<T> | undefined;
//     onPointerUpCapture?: PointerEventHandler<T> | undefined;
//     onPointerCancel?: PointerEventHandler<T> | undefined;
//     onPointerCancelCapture?: PointerEventHandler<T> | undefined;
//     onPointerEnter?: PointerEventHandler<T> | undefined;
//     onPointerEnterCapture?: PointerEventHandler<T> | undefined;
//     onPointerLeave?: PointerEventHandler<T> | undefined;
//     onPointerLeaveCapture?: PointerEventHandler<T> | undefined;
//     onPointerOver?: PointerEventHandler<T> | undefined;
//     onPointerOverCapture?: PointerEventHandler<T> | undefined;
//     onPointerOut?: PointerEventHandler<T> | undefined;
//     onPointerOutCapture?: PointerEventHandler<T> | undefined;
//     onGotPointerCapture?: PointerEventHandler<T> | undefined;
//     onGotPointerCaptureCapture?: PointerEventHandler<T> | undefined;
//     onLostPointerCapture?: PointerEventHandler<T> | undefined;
//     onLostPointerCaptureCapture?: PointerEventHandler<T> | undefined;

//     // UI Events
//     onScroll?: UIEventHandler<T> | undefined;
//     onScrollCapture?: UIEventHandler<T> | undefined;

//     // Wheel Events
//     onWheel?: WheelEventHandler<T> | undefined;
//     onWheelCapture?: WheelEventHandler<T> | undefined;

//     // Animation Events
//     onAnimationStart?: AnimationEventHandler<T> | undefined;
//     onAnimationStartCapture?: AnimationEventHandler<T> | undefined;
//     onAnimationEnd?: AnimationEventHandler<T> | undefined;
//     onAnimationEndCapture?: AnimationEventHandler<T> | undefined;
//     onAnimationIteration?: AnimationEventHandler<T> | undefined;
//     onAnimationIterationCapture?: AnimationEventHandler<T> | undefined;

//     // Transition Events
//     onTransitionEnd?: TransitionEventHandler<T> | undefined;
//     onTransitionEndCapture?: TransitionEventHandler<T> | undefined;
// }
