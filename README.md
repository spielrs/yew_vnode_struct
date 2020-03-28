# Yew VNode Structure

This module contains the implementation to get all the VNode structure.
it is useful for testing and debuging.
For example we want to know what contain this VNode:

```rust
let example = html! {
    <div id="example">{"example"}</div>
};
```

Now we use VNodeStruct to get the complete Vnode button and log the result:

```rust
let vnode_example = VNodeStruct::new(example);

console.log(&format!("{:#?}", example));
```

We will get this in the console:

```
VNodeStruct {
    vtag: Some(
        VTagStruct {
            reference: None,
            attributes: {
                "id": "example",
            },
            classes: Classes {
                set: {},
            },
            value: None,
            kind: None,
            checked: false,
            node_ref: NodeRef(
                RefCell {
                    value: NodeRefInner {
                        node: None,
                        link: None,
                    },
                },
            ),
        },
    ),
    vlist: None,
    vtext: None,
    vcomp: None,
    vref: None,
    children: Some(
        [
            VNodeStruct {
                vtag: None,
                vlist: Some(
                    VList {
                        children: [
                            VText { text: example },
                        ],
                        elide_placeholder: true,
                    },
                ),
                vtext: None,
                vcomp: None,
                vref: None,
                children: Some(
                    [
                        VNodeStruct {
                            vtag: None,
                            vlist: None,
                            vtext: Some(
                                VText { text: example },
                            ),
                            vcomp: None,
                            vref: None,
                            children: None,
                        },
                    ],
                ),
            },
        ],
    ),
}
```

### Run unit tests

`cargo test`

### License

Spiel Request is MIT licensed. See [license](LICENSE)