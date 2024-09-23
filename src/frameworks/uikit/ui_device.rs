/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIDevice`.

use crate::dyld::ConstantExports;
use crate::dyld::HostConstant;
use crate::frameworks::foundation::ns_string;
use crate::frameworks::foundation::{NSInteger, NSUInteger};
use crate::mem::MutPtr;
use crate::objc::{id, msg, nil, objc_classes, ClassExports, TrivialHostObject};
use crate::window::DeviceOrientation;

pub const UIDeviceOrientationDidChangeNotification: &str =
    "UIDeviceOrientationDidChangeNotification";

pub type UIDeviceOrientation = NSInteger;
#[allow(dead_code)]
pub const UIDeviceOrientationUnknown: UIDeviceOrientation = 0;
pub const UIDeviceOrientationPortrait: UIDeviceOrientation = 1;
#[allow(dead_code)]
pub const UIDeviceOrientationPortraitUpsideDown: UIDeviceOrientation = 2;
pub const UIDeviceOrientationLandscapeLeft: UIDeviceOrientation = 3;
pub const UIDeviceOrientationLandscapeRight: UIDeviceOrientation = 4;
#[allow(dead_code)]
pub const UIDeviceOrientationFaceUp: UIDeviceOrientation = 5;
#[allow(dead_code)]
pub const UIDeviceOrientationFaceDown: UIDeviceOrientation = 6;

#[derive(Default)]
pub struct State {
    current_device: Option<id>,
}

pub const CONSTANTS: ConstantExports = &[(
    "_UIDeviceOrientationDidChangeNotification",
    HostConstant::NSString(UIDeviceOrientationDidChangeNotification),
)];

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIDevice: NSObject

+ (id)currentDevice {
    if let Some(device) = env.framework_state.uikit.ui_device.current_device {
        device
    } else {
        let new = env.objc.alloc_static_object(
            this,
            Box::new(TrivialHostObject),
            &mut env.mem
        );
        env.framework_state.uikit.ui_device.current_device = Some(new);
        new
    }
}

- (())beginGeneratingDeviceOrientationNotifications {
    log!("TODO: beginGeneratingDeviceOrientationNotifications");
}
- (())endGeneratingDeviceOrientationNotifications {
    log!("TODO: endGeneratingDeviceOrientationNotifications");
}

- (())setOrientation:(bool)orientation {
    log!("TODO: setOrientation:{}", orientation);
}

- (())setBatteryMonitoringEnabled:(bool)enabled {
    log!("TODO: setBatteryMonitoringEnabled:{}", enabled);
}

- (id)model {
    // TODO: Hardcoded to iPhone for now
    ns_string::get_static_str(env, "iPhone")
}

- (id)name {
    // TODO: Hardcoded to iPhone for now
    ns_string::get_static_str(env, "iPhone")
}

- (id)methodSignatureForSelector:(NSUInteger)selector {
    msg![env; this init]
}

- (id)systemName {
    ns_string::get_static_str(env, "iPhone OS")
}

- (id)localizedModel {
    ns_string::get_static_str(env, "iPhone")
}

// NSString
- (id)systemVersion {
    ns_string::get_static_str(env, "2.0")
}

- (id)userInterfaceIdiom {
    nil
}

- (id)uniqueIdentifier {
    // Aspen Simulator returns (null) here
    // TODO: what should be a correct value?
    ns_string::get_static_str(env, "touchHLEdevice")
}

- (bool)isMultitaskingSupported {
    false
}

- (UIDeviceOrientation)orientation {
    match env.window().current_rotation() {
        DeviceOrientation::Portrait => UIDeviceOrientationPortrait,
        DeviceOrientation::LandscapeLeft => UIDeviceOrientationLandscapeLeft,
        DeviceOrientation::LandscapeRight => UIDeviceOrientationLandscapeRight
    }
}

@end

@implementation NSURLConnection: NSObject
+ (id)sendSynchronousRequest:(id)request
           returningResponse:(MutPtr<id>)response
                       error:(MutPtr<id>)error {
    nil
}
@end

@implementation NSHTTPURLResponse: NSObject

@end

};
