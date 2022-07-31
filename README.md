# Yew DOM Attributes
If you're making your own custom Yew components, DOM attributes are no problem: you can simply set
the ones that you need for your case.

However, if you're making a component library, you'd want your users to be able to set any DOM
attribute regardless of whether you explicitly define it. This crate aims to solve this problem.

## How To Use It
First, create a Yew component. A simple implementation is shown below:

```rust
#[derive(Clone, PartialEq, Properties)]
struct MyButtonProps {
    #[prop_or_default] 
    text: String,
    #[prop_or_default]
    disabled: bool,
}

struct MyButton;

impl Component for MyButton {
    type Message = ();
    type Properties = MyButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button 
              disabled={ctx.props().disabled}>
                {ctx.props().text.clone()}
            </button>
        }
    }
}
```

Next, add MiscAttrs to your props struct:

```rust
#[derive(Clone, PartialEq, Properties)]
struct MyButtonProps {
    #[prop_or_default] 
    text: String,
    #[prop_or_default]
    disabled: bool,
    #[prop_or_default]
    misc_attrs: MiscAttrs
}
```

We need access to the underlying DOM element, so add a NodeRef to your MyButton struct:

```rust
struct MyButton {
  node_ref: NodeRef,
}
```

Add the ref to the props too so your component's users can set it manually if they want,
giving them access to the underlying DOM node as well:

```rust
#[derive(Clone, PartialEq, Properties)]
struct MyButtonProps {
    #[prop_or_default]
    text: String,
    disabled: bool,
    #[prop_or_default]
    misc_attrs: MiscAttrs,
    #[prop_or_default]
    pub node_ref: NodeRef,
}
```

Add a create function to handle the NodeRef:

```rust
impl Component for MyButton {
  ...

  fn create(ctx: &Context<Self>) -> Self {
      Self {
          node_ref: ctx.props().node_ref.clone(),
      }
  }
}
```

Finally, we add a rendered function, where we inject our DOM attributes:

```rust
impl Component for MyButton {
  ...

  fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
    ctx.props().misc_attrs.inject(&self.node_ref);
  }
}
```


## Goals
Currently the crate only provides a MiscAttrs struct, which can be used to inject an arbitrary set of
key-value pairs, as well as boolean attributes.

However, it would be nice to provide fully-typed structs for different types of base HTML elements.
As an example of what this might look like, we have the AriaAttributes struct, which has a complete
implementation of aria attributes and behaves the same as MiscAttrs but is strongly typed, making
it easier for consumers of a component to get the right attribute.