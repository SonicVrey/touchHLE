/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIActivityIndicatorView`.

use crate::frameworks::foundation::{NSInteger, NSUInteger};
use crate::objc::{id, msg, nil, ClassExports};
use crate::objc_classes;

type UIActivityIndicatorViewStyle = NSInteger;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIActivityIndicatorView: UIView

- (id)initWithActivityIndicatorStyle:(UIActivityIndicatorViewStyle)_style {
    // TODO: proper init
    msg![env; this init]
}

- (())setHidesWhenStopped:(bool)hides {
    log!("TODO: setHidesWhenStopped:{}", hides);
}

- (())setActivityIndicatorViewStyle :(bool)activity {
    log!("TODO: setActivityIndicatorViewStyle:{}", activity);
}

- (())sizeThatFits:(bool)fits {
    log!("TODO: sizeThatFits:{}", fits);
}
- (id)sizeToFit {
    nil
}

- (())startAnimating {
    log!("TODO: [(UIActivityIndicatorView*){:?} startAnimating", this);
}

- (())stopAnimating {
    log!("TODO: [(UIActivityIndicatorView*){:?} stopAnimating", this);
}

@end

@implementation UIActionSheet: NSObject
- (id)addButtonWithTitle:(NSUInteger)_title {
    msg![env; this init]
}

- (id)showInView:(NSUInteger)_view {
    msg![env; this init]
}

- (())setActionSheetStyle:(bool)action {
    log!("TODO: setActionSheetStyle:{}", action);
}

- (())setTitle:(bool)title {
    log!("TODO: setTitle:{}", title);
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

- (())initWithTitle:(NSInteger)title delegate:(bool)_delegate cancelButtonTitle:(bool)_button destructiveButtonTitle:(bool)_destructive otherButtonTitles:(bool)_titles {
    // TODO
}

@end

};
