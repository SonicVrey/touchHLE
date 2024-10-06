use crate::frameworks::foundation::NSUInteger;
use crate::objc::{id, msg, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIProgressView: UIView

- (id)initWithProgressViewStyle:(NSUInteger)style {
    msg![env; this init]
}

- (())setProgressViewStyle:(bool)style {
    log!("TODO: setProgressViewStyle:{}", style);
}

- (())setProgress:(bool)progress {
    log!("TODO: setProgress:{}", progress);
}

@end

};
