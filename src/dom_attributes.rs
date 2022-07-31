use js_sys::Function;
use wasm_bindgen::{
    closure::WasmClosure,
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use web_sys::{console, Element, MouseEvent};
use yew::{html::onabort::Event, Callback, Properties};

use crate::attribute_collection::AttributeCollection;

#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct DOMAttributes {
    // interface DOMAttributes<T> {

    //   // Clipboard Events
    //   onCopy?: ClipboardEventHandler<T> | undefined;
    //   onCopyCapture?: ClipboardEventHandler<T> | undefined;
    //   onCut?: ClipboardEventHandler<T> | undefined;
    //   onCutCapture?: ClipboardEventHandler<T> | undefined;
    //   onPaste?: ClipboardEventHandler<T> | undefined;
    //   onPasteCapture?: ClipboardEventHandler<T> | undefined;

    //   // Composition Events
    //   onCompositionEnd?: CompositionEventHandler<T> | undefined;
    //   onCompositionEndCapture?: CompositionEventHandler<T> | undefined;
    //   onCompositionStart?: CompositionEventHandler<T> | undefined;
    //   onCompositionStartCapture?: CompositionEventHandler<T> | undefined;
    //   onCompositionUpdate?: CompositionEventHandler<T> | undefined;
    //   onCompositionUpdateCapture?: CompositionEventHandler<T> | undefined;

    //   // Focus Events
    //   onFocus?: FocusEventHandler<T> | undefined;
    //   onFocusCapture?: FocusEventHandler<T> | undefined;
    //   onBlur?: FocusEventHandler<T> | undefined;
    //   onBlurCapture?: FocusEventHandler<T> | undefined;

    //   // Form Events
    //   onChange?: FormEventHandler<T> | undefined;
    //   onChangeCapture?: FormEventHandler<T> | undefined;
    //   onBeforeInput?: FormEventHandler<T> | undefined;
    //   onBeforeInputCapture?: FormEventHandler<T> | undefined;
    //   onInput?: FormEventHandler<T> | undefined;
    //   onInputCapture?: FormEventHandler<T> | undefined;
    //   onReset?: FormEventHandler<T> | undefined;
    //   onResetCapture?: FormEventHandler<T> | undefined;
    //   onSubmit?: FormEventHandler<T> | undefined;
    //   onSubmitCapture?: FormEventHandler<T> | undefined;
    //   onInvalid?: FormEventHandler<T> | undefined;
    //   onInvalidCapture?: FormEventHandler<T> | undefined;

    //   // Image Events
    //   onLoad?: ReactEventHandler<T> | undefined;
    //   onLoadCapture?: ReactEventHandler<T> | undefined;
    //   onError?: ReactEventHandler<T> | undefined; // also a Media Event
    //   onErrorCapture?: ReactEventHandler<T> | undefined; // also a Media Event

    //   // Keyboard Events
    //   onKeyDown?: KeyboardEventHandler<T> | undefined;
    //   onKeyDownCapture?: KeyboardEventHandler<T> | undefined;
    //   /** @deprecated */
    //   onKeyPress?: KeyboardEventHandler<T> | undefined;
    //   /** @deprecated */
    //   onKeyPressCapture?: KeyboardEventHandler<T> | undefined;
    //   onKeyUp?: KeyboardEventHandler<T> | undefined;
    //   onKeyUpCapture?: KeyboardEventHandler<T> | undefined;

    //   // Media Events
    #[prop_or_default]
    pub onabort: Option<Callback<Event>>, //   onAbortCapture?: ReactEventHandler<T> | undefined;
    //   onCanPlay?: ReactEventHandler<T> | undefined;
    //   onCanPlayCapture?: ReactEventHandler<T> | undefined;
    //   onCanPlayThrough?: ReactEventHandler<T> | undefined;
    //   onCanPlayThroughCapture?: ReactEventHandler<T> | undefined;
    //   onDurationChange?: ReactEventHandler<T> | undefined;
    //   onDurationChangeCapture?: ReactEventHandler<T> | undefined;
    //   onEmptied?: ReactEventHandler<T> | undefined;
    //   onEmptiedCapture?: ReactEventHandler<T> | undefined;
    //   onEncrypted?: ReactEventHandler<T> | undefined;
    //   onEncryptedCapture?: ReactEventHandler<T> | undefined;
    //   onEnded?: ReactEventHandler<T> | undefined;
    //   onEndedCapture?: ReactEventHandler<T> | undefined;
    //   onLoadedData?: ReactEventHandler<T> | undefined;
    //   onLoadedDataCapture?: ReactEventHandler<T> | undefined;
    //   onLoadedMetadata?: ReactEventHandler<T> | undefined;
    //   onLoadedMetadataCapture?: ReactEventHandler<T> | undefined;
    //   onLoadStart?: ReactEventHandler<T> | undefined;
    //   onLoadStartCapture?: ReactEventHandler<T> | undefined;
    //   onPause?: ReactEventHandler<T> | undefined;
    //   onPauseCapture?: ReactEventHandler<T> | undefined;
    //   onPlay?: ReactEventHandler<T> | undefined;
    //   onPlayCapture?: ReactEventHandler<T> | undefined;
    //   onPlaying?: ReactEventHandler<T> | undefined;
    //   onPlayingCapture?: ReactEventHandler<T> | undefined;
    //   onProgress?: ReactEventHandler<T> | undefined;
    //   onProgressCapture?: ReactEventHandler<T> | undefined;
    //   onRateChange?: ReactEventHandler<T> | undefined;
    //   onRateChangeCapture?: ReactEventHandler<T> | undefined;
    //   onSeeked?: ReactEventHandler<T> | undefined;
    //   onSeekedCapture?: ReactEventHandler<T> | undefined;
    //   onSeeking?: ReactEventHandler<T> | undefined;
    //   onSeekingCapture?: ReactEventHandler<T> | undefined;
    //   onStalled?: ReactEventHandler<T> | undefined;
    //   onStalledCapture?: ReactEventHandler<T> | undefined;
    //   onSuspend?: ReactEventHandler<T> | undefined;
    //   onSuspendCapture?: ReactEventHandler<T> | undefined;
    //   onTimeUpdate?: ReactEventHandler<T> | undefined;
    //   onTimeUpdateCapture?: ReactEventHandler<T> | undefined;
    //   onVolumeChange?: ReactEventHandler<T> | undefined;
    //   onVolumeChangeCapture?: ReactEventHandler<T> | undefined;
    //   onWaiting?: ReactEventHandler<T> | undefined;
    //   onWaitingCapture?: ReactEventHandler<T> | undefined;

    //   // MouseEvents
    //   onAuxClick?: MouseEventHandler<T> | undefined;
    //   onAuxClickCapture?: MouseEventHandler<T> | undefined;
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>, //   onClickCapture?: MouseEventHandler<T> | undefined;
                                               //   onContextMenu?: MouseEventHandler<T> | undefined;
                                               //   onContextMenuCapture?: MouseEventHandler<T> | undefined;
                                               //   onDoubleClick?: MouseEventHandler<T> | undefined;
                                               //   onDoubleClickCapture?: MouseEventHandler<T> | undefined;
                                               //   onDrag?: DragEventHandler<T> | undefined;
                                               //   onDragCapture?: DragEventHandler<T> | undefined;
                                               //   onDragEnd?: DragEventHandler<T> | undefined;
                                               //   onDragEndCapture?: DragEventHandler<T> | undefined;
                                               //   onDragEnter?: DragEventHandler<T> | undefined;
                                               //   onDragEnterCapture?: DragEventHandler<T> | undefined;
                                               //   onDragExit?: DragEventHandler<T> | undefined;
                                               //   onDragExitCapture?: DragEventHandler<T> | undefined;
                                               //   onDragLeave?: DragEventHandler<T> | undefined;
                                               //   onDragLeaveCapture?: DragEventHandler<T> | undefined;
                                               //   onDragOver?: DragEventHandler<T> | undefined;
                                               //   onDragOverCapture?: DragEventHandler<T> | undefined;
                                               //   onDragStart?: DragEventHandler<T> | undefined;
                                               //   onDragStartCapture?: DragEventHandler<T> | undefined;
                                               //   onDrop?: DragEventHandler<T> | undefined;
                                               //   onDropCapture?: DragEventHandler<T> | undefined;
                                               //   onMouseDown?: MouseEventHandler<T> | undefined;
                                               //   onMouseDownCapture?: MouseEventHandler<T> | undefined;
                                               //   onMouseEnter?: MouseEventHandler<T> | undefined;
                                               //   onMouseLeave?: MouseEventHandler<T> | undefined;
                                               //   onMouseMove?: MouseEventHandler<T> | undefined;
                                               //   onMouseMoveCapture?: MouseEventHandler<T> | undefined;
                                               //   onMouseOut?: MouseEventHandler<T> | undefined;
                                               //   onMouseOutCapture?: MouseEventHandler<T> | undefined;
                                               //   onMouseOver?: MouseEventHandler<T> | undefined;
                                               //   onMouseOverCapture?: MouseEventHandler<T> | undefined;
                                               //   onMouseUp?: MouseEventHandler<T> | undefined;
                                               //   onMouseUpCapture?: MouseEventHandler<T> | undefined;

                                               //   // Selection Events
                                               //   onSelect?: ReactEventHandler<T> | undefined;
                                               //   onSelectCapture?: ReactEventHandler<T> | undefined;

                                               //   // Touch Events
                                               //   onTouchCancel?: TouchEventHandler<T> | undefined;
                                               //   onTouchCancelCapture?: TouchEventHandler<T> | undefined;
                                               //   onTouchEnd?: TouchEventHandler<T> | undefined;
                                               //   onTouchEndCapture?: TouchEventHandler<T> | undefined;
                                               //   onTouchMove?: TouchEventHandler<T> | undefined;
                                               //   onTouchMoveCapture?: TouchEventHandler<T> | undefined;
                                               //   onTouchStart?: TouchEventHandler<T> | undefined;
                                               //   onTouchStartCapture?: TouchEventHandler<T> | undefined;

                                               //   // Pointer Events
                                               //   onPointerDown?: PointerEventHandler<T> | undefined;
                                               //   onPointerDownCapture?: PointerEventHandler<T> | undefined;
                                               //   onPointerMove?: PointerEventHandler<T> | undefined;
                                               //   onPointerMoveCapture?: PointerEventHandler<T> | undefined;
                                               //   onPointerUp?: PointerEventHandler<T> | undefined;
                                               //   onPointerUpCapture?: PointerEventHandler<T> | undefined;
                                               //   onPointerCancel?: PointerEventHandler<T> | undefined;
                                               //   onPointerCancelCapture?: PointerEventHandler<T> | undefined;
                                               //   onPointerEnter?: PointerEventHandler<T> | undefined;
                                               //   onPointerEnterCapture?: PointerEventHandler<T> | undefined;
                                               //   onPointerLeave?: PointerEventHandler<T> | undefined;
                                               //   onPointerLeaveCapture?: PointerEventHandler<T> | undefined;
                                               //   onPointerOver?: PointerEventHandler<T> | undefined;
                                               //   onPointerOverCapture?: PointerEventHandler<T> | undefined;
                                               //   onPointerOut?: PointerEventHandler<T> | undefined;
                                               //   onPointerOutCapture?: PointerEventHandler<T> | undefined;
                                               //   onGotPointerCapture?: PointerEventHandler<T> | undefined;
                                               //   onGotPointerCaptureCapture?: PointerEventHandler<T> | undefined;
                                               //   onLostPointerCapture?: PointerEventHandler<T> | undefined;
                                               //   onLostPointerCaptureCapture?: PointerEventHandler<T> | undefined;

                                               //   // UI Events
                                               //   onScroll?: UIEventHandler<T> | undefined;
                                               //   onScrollCapture?: UIEventHandler<T> | undefined;

                                               //   // Wheel Events
                                               //   onWheel?: WheelEventHandler<T> | undefined;
                                               //   onWheelCapture?: WheelEventHandler<T> | undefined;

                                               //   // Animation Events
                                               //   onAnimationStart?: AnimationEventHandler<T> | undefined;
                                               //   onAnimationStartCapture?: AnimationEventHandler<T> | undefined;
                                               //   onAnimationEnd?: AnimationEventHandler<T> | undefined;
                                               //   onAnimationEndCapture?: AnimationEventHandler<T> | undefined;
                                               //   onAnimationIteration?: AnimationEventHandler<T> | undefined;
                                               //   onAnimationIterationCapture?: AnimationEventHandler<T> | undefined;

                                               //   // Transition Events
                                               //   onTransitionEnd?: TransitionEventHandler<T> | undefined;
                                               //   onTransitionEndCapture?: TransitionEventHandler<T> | undefined;
                                               // }
}

impl AttributeCollection for DOMAttributes {
    fn inject(&self, node_ref: &yew::NodeRef) {
        if let Some(elem) = node_ref.cast::<Element>() {
            self.inject_listener(&elem, &self.onclick);
        }
    }
}

impl DOMAttributes {
    fn inject_listener(&self, elem: &Element, listener: &Option<Callback<MouseEvent>>) {
        if let Some(listener) = listener.clone() {
            let listener = listener.clone();
            let closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
                listener.emit(event);
            });
            elem.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }
}
