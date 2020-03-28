//! This module contains the implementation to get all the VNode structure.
//! it is useful for testing and debuging. For example to know what contain this VNode
//! we use VNodeStruct to get the complete Vnode button and log the result:
//!
//! #Example
//!
//! ```
//! extern crate yew;
//! extern crate yew_vnode_struct;
//!
//! use yew::html;
//! use yew_vnode_struct::VNodeStruct;
//!
//! let example = html! {
//!     <div id="example">{"example"}</div>
//! };
//!
//! let vnode_example = VNodeStruct::new(example.clone());
//!
//! println!("{:#?}", example);
//! ```
//!
//! We will get this structure:
//!
//! ```ignore
//! VNodeStruct {
//!     vtag: Some(
//!         VTagStruct {
//!             reference: None,
//!             attributes: {
//!                 "id": "example",
//!             },
//!             classes: Classes {
//!                 set: {},
//!             },
//!             value: None,
//!             kind: None,
//!             checked: false,
//!             node_ref: NodeRef(
//!                 RefCell {
//!                     value: NodeRefInner {
//!                         node: None,
//!                         link: None,
//!                     },
//!                 },
//!             ),
//!         },
//!     ),
//!     vlist: None,
//!     vtext: None,
//!     vcomp: None,
//!     vref: None,
//!     children: Some(
//!         [
//!             VNodeStruct {
//!                 vtag: None,
//!                 vlist: Some(
//!                     VList {
//!                         children: [
//!                             VText { text: example },
//!                         ],
//!                         elide_placeholder: true,
//!                     },
//!                 ),
//!                 vtext: None,
//!                 vcomp: None,
//!                 vref: None,
//!                 children: Some(
//!                     [
//!                         VNodeStruct {
//!                             vtag: None,
//!                             vlist: None,
//!                             vtext: Some(
//!                                 VText { text: example },
//!                             ),
//!                             vcomp: None,
//!                             vref: None,
//!                             children: None,
//!                         },
//!                     ],
//!                 ),
//!             },
//!         ],
//!     ),
//! }
//! ```

pub mod vnode_struct;

pub use vnode_struct::VNodeStruct;
