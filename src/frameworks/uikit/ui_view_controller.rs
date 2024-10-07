/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIViewController`.

use crate::frameworks::foundation::ns_string::{get_static_str, to_rust_string};
use crate::frameworks::foundation::{NSInteger, NSUInteger};
use crate::objc::{
    id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr,
};

#[derive(Default)]
pub struct UIViewControllerHostObject {
    view: id,
}
impl HostObject for UIViewControllerHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIViewController: UIResponder

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIViewControllerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)copyWithZone:(NSUInteger)_zone {
    msg![env; this init]
}

// TODO: this should be the designated initializer
- (id)initWithNibName:(id)nibNameOrNil // NSString *
               bundle:(id)nibBundleOrNil { // NSBundle *
    // assert!(nibBundleOrNil == nil);
    let x = to_rust_string(env, nibNameOrNil);
    log!("initWithNibName: {}", x);
    let bundle: id = msg_class![env; NSBundle mainBundle];
    let type_: id = get_static_str(env, "nib");
    let path: id = msg![env; bundle pathForResource:nibNameOrNil ofType:type_];

    // assert!(msg![env; path isAbsolutePath]);
    let ns_data: id = msg_class![env; NSData dataWithContentsOfFile:path];
    // assert!(ns_data != nil);

    let unarchiver = msg_class![env; NSKeyedUnarchiver alloc];
    let unarchiver = msg![env; unarchiver initForReadingWithData:ns_data];

    // We don't need to do anything with the list of objects, but deserializing
    // it ensures everything else is deserialized.
    let objects_key = get_static_str(env, "UINibObjectsKey");
    let _objects: id = msg![env; unarchiver decodeObjectForKey:objects_key];

    msg![env; this initWithCoder:unarchiver]
}

- (id)initWithCoder:(id)coder {
    let key_ns_string = get_static_str(env, "UIView");
    let view: id = msg![env; coder decodeObjectForKey:key_ns_string];

    () = msg![env; this setView:view];

    this
}

- (id)initWithStyle:(NSUInteger)_style {
    msg![env; this init]
}

- (id)closeConnection {
    nil
}

- (id)navigationController {
    nil
}

- (id)navigationItem {
    nil
}

- (id)tabBarItem {
    nil
}

- (())dealloc {
    let &UIViewControllerHostObject { view } = env.objc.borrow(this);

    release(env, view);

    env.objc.dealloc_object(this, &mut env.mem);
}

- (())loadView {
    // TODO: Check if the UIViewController has an associated nib file and load
    // the view from there instead if it does
    let view: id = msg_class![env; UIView alloc];
    let view: id = msg![env; view init];
    () = msg![env; this setView: view];
}
- (())setView:(id)new_view { // UIView*
    let host_obj = env.objc.borrow_mut::<UIViewControllerHostObject>(this);
    let old_view = std::mem::replace(&mut host_obj.view, new_view);
    retain(env, new_view);
    release(env, old_view);
}
- (id)view {
    let view = env.objc.borrow_mut::<UIViewControllerHostObject>(this).view;
    if view == nil {
        () = msg![env; this loadView];
        let view = env.objc.borrow_mut::<UIViewControllerHostObject>(this).view;
        view
    } else {
        view
    }
}

- (())setTitle:(id)title {
    let title = to_rust_string(env, title);
    log!("TODO: [(UIViewController*){:?} setTitle:{}]", this, title); // TODO
}

- (())setTabBarItem:(bool)item {
    log!("TODO: setTabBarItem:{}", item);
}

- (())setEditing:(bool)editing {
    log!("TODO: [(UIViewController*){:?} setEditing:{}]", this, editing); // TODO
}

- (())setWantsFullScreenLayout:(bool)screen {
    log!("TODO: setWantsFullScreenLayout:{}", screen);
}

- (())dismissModalViewControllerAnimated:(bool)animated {
    log!("TODO: [(UIViewController*){:?} dismissModalViewControllerAnimated:{}]", this, animated); // TODO
}

- (())viewWillAppear:(bool)animated {
    log!("TODO: [(UIViewController*){:?} viewWillAppear:{}]", this, animated); // TODO
}

- (())viewDidAppear:(bool)animated {
    log!("TODO: [(UIViewController*){:?} viewDidAppear:{}]", this, animated); // TODO
}

- (())performSelectorInBackground:(NSInteger)selector withObject:(bool)_object {
    // TODO
}

- (())performSelector:(NSInteger)selector withObject:(bool)_object afterDelay:(bool)_delay  {
    // TODO
}

@end

@implementation BallFightSettingsViewController: UIViewController
- (id)navigationController {
    nil
}

@end

@implementation CCViewController:  UIViewController
- (())performSelectorInBackground:(NSInteger)selector withObject:(bool)_object {
    // TODO
}

@end

@implementation DNSViewController: UIViewController
- (id)closeConnection {
    nil
}

@end

@implementation GKPeerPickerController: UIViewController
- (id)show {
    nil
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

@end

@implementation GunFighterViewController: UIViewController
- (())performSelector:(NSInteger)selector withObject:(bool)_object afterDelay:(bool)_delay  {
    // TODO
}

@end

@implementation HomeViewController: UIViewController
- (())setTabBarItem:(bool)item {
    log!("TODO: setTabBarItem:{}", item);
}

@end

@implementation MyViewController: UIViewController
- (id)tabBarItem {
    nil
}

@end

@implementation ProjectListViewController: UIViewController
- (id)initWithStyle:(NSUInteger)_style {
    msg![env; this init]
}

@end

@implementation ReachabilityQuery: UIViewController
- (id)copyWithZone:(NSUInteger)_zone {
    msg![env; this init]
}

@end

@implementation Test3DViewController: UIViewController
- (())setWantsFullScreenLayout:(bool)screen {
    log!("TODO: setWantsFullScreenLayout:{}", screen);
}

@end

@implementation GameViewController: UIViewController
- (id)navigationItem {
    nil
}

@end

};
