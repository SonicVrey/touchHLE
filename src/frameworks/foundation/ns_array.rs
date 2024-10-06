/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The `NSArray` class cluster, including `NSMutableArray`.

use super::ns_enumerator::{fast_enumeration_helper, NSFastEnumerationState};
use super::ns_property_list_serialization::deserialize_plist_from_file;
use super::{ns_keyed_unarchiver, ns_string, ns_url, NSNotFound, NSOrderedAscending, NSOrderedDescending, NSOrderedSame, NSInteger, NSUInteger};
use crate::abi::DotDotDot;
use crate::frameworks::foundation::ns_dictionary::DictionaryHostObject;
use crate::fs::GuestPath;
use crate::mem::{MutPtr, MutVoidPtr};
use crate::objc::{
    autorelease, id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject,
    NSZonePtr,
};
use crate::Environment;
use std::cmp::{min, Ordering};
use std::mem;
use std::ops::Add;

struct ObjectEnumeratorHostObject {
    iterator: std::vec::IntoIter<id>,
}
impl HostObject for ObjectEnumeratorHostObject {}

/// Belongs to _touchHLE_NSArray
struct ArrayHostObject {
    array: Vec<id>,
}
impl HostObject for ArrayHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// NSArray is an abstract class. A subclass must provide:
// - (NSUInteger)count;
// - (id)objectAtIndex:(NSUInteger)index;
// We can pick whichever subclass we want for the various alloc methods.
// For the time being, that will always be _touchHLE_NSArray.
@implementation NSArray: NSObject

+ (id)allocWithZone:(NSZonePtr)zone {
    // NSArray might be subclassed by something which needs allocWithZone:
    // to have the normal behaviour. Unimplemented: call superclass alloc then.
    assert!(this == env.objc.get_known_class("NSArray", &mut env.mem));
    msg_class![env; _touchHLE_NSArray allocWithZone:zone]
}

// These probably comes from some category related to plists.
+ (id)arrayWithContentsOfFile:(id)path { // NSString*
    let path = ns_string::to_rust_string(env, path);
    let res = deserialize_plist_from_file(
        env,
        GuestPath::new(&path),
        /* array_expected: */ true,
    );
    autorelease(env, res)
}
+ (id)arrayWithContentsOfURL:(id)url { // NSURL*
    let path = ns_url::to_rust_path(env, url);
    let res = deserialize_plist_from_file(env, &path, /* array_expected: */ true);
    autorelease(env, res)
}

+ (id)arrayWithArray:(id)other {
    let new = msg![env; this alloc];
    let new = msg![env; new initWithArray: other];
    autorelease(env, new)
}

+ (id)arrayWithObjects:(id)first, ...rest {
    let new = msg_class![env; NSArray alloc];
    from_va_args(env, new, first, rest);
    autorelease(env, new)
}

+ (id)array {
    let new = msg![env; this alloc];
    let new = msg![env; new init];
    autorelease(env, new)
}

+ (id)arrayWithObject:(id)obj {
    let new = msg![env; this alloc];
    retain(env, obj);
    env.objc.borrow_mut::<ArrayHostObject>(new).array.push(obj);
    autorelease(env, new)
}

// These probably comes from some category related to plists.
- (id)initWithContentsOfFile:(id)path { // NSString*
    release(env, this);
    let path = ns_string::to_rust_string(env, path);
    deserialize_plist_from_file(
        env,
        GuestPath::new(&path),
        /* array_expected: */ true,
    )
}
- (id)initWithContentsOfURL:(id)url { // NSURL*
    release(env, this);
    let path = ns_url::to_rust_path(env, url);
    deserialize_plist_from_file(env, &path, /* array_expected: */ true)
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    retain(env, this)
}

- (NSUInteger)indexOfObject:(id)object {
    let count: NSUInteger = msg![env; this count];
    for i in 0..count {
        let curr_object: id = msg![env; this objectAtIndex:i];
        let equal: bool = msg![env; object isEqual:curr_object];
        if equal {
            return i;
        }
    }
    NSNotFound as NSUInteger
}

- (id)lastObject {
    let size: NSUInteger = msg![env; this count];
    if size == 0 {
        return nil;
    }
    msg![env; this objectAtIndex: (size - 1)]
}

- (id)initWithArray:(id)other {
    let size: NSUInteger = msg![env; other count];
    let mut v = Vec::with_capacity(size as usize);
    for i in 0..size {
        let obj = msg![env; other objectAtIndex: i];
        v.push(retain(env, obj));
    }
    env.objc.borrow_mut::<ArrayHostObject>(this).array = v;
    this
}

- (id)initWithObjects:(id)first, ...rest {
    from_va_args(env, this, first, rest);
    this
}

- (NSUInteger)countByEnumeratingWithState:(MutPtr<NSFastEnumerationState>)state
                                  objects:(MutPtr<id>)stackbuf
                                    count:(NSUInteger)len {
    let host_object = env.objc.borrow::<ArrayHostObject>(this);

    if host_object.array.is_empty() {
        return 0;
    }

    let NSFastEnumerationState {
        state: cur_idx,
        ..
    } = env.mem.read(state);

    let this_round = min(host_object.array.len() as u32 - cur_idx, len);
    if cur_idx == 0 {
        env.mem.write(state, NSFastEnumerationState {
            state: 0,
            items_ptr: stackbuf,
            mutations_ptr: this.cast(),
            extra: Default::default(),
        });
    }
    env.mem.write(state.cast(), (cur_idx + this_round) as NSUInteger);
    for i in 0..this_round {
        env.mem.write(stackbuf.add(i), host_object.array[(cur_idx + i) as usize]);
    }
    this_round
}

-(id)mutableCopyWithZone:(NSZonePtr)_zone {
    let new = msg_class![env; NSMutableArray alloc];
    msg![env; new initWithArray: this]
}

-(id)mutableCopy {
    msg![env; this mutableCopyWithZone:(MutVoidPtr::null())]
}

- (NSUInteger)indexOfObject:(id)needle {
    let objs = env.objc.borrow::<ArrayHostObject>(this).array.clone();
    for (i, &obj) in objs.iter().enumerate() {
        if msg![env; needle isEqual: obj] {
            return i as NSUInteger;
        }
    }
    NSNotFound.try_into().unwrap()
}

-(bool)containsObject:(id)needle {
    let idx: NSUInteger = msg![env; this indexOfObject: needle];
    idx != NSNotFound.try_into().unwrap()
}

- (id)objectEnumerator { // NSEnumerator*
    let array_host_object: &mut ArrayHostObject = env.objc.borrow_mut(this);
    let vec = array_host_object.array.to_vec();
    let host_object = Box::new(ObjectEnumeratorHostObject {
        iterator: vec.into_iter(),
    });
    let class = env.objc.get_known_class("_touchHLE_NSArray_ObjectEnumerator", &mut env.mem);
    let enumerator = env.objc.alloc_object(class, host_object, &mut env.mem);
    autorelease(env, enumerator)
}

-(id)sortedArrayUsingDescriptors:(id)desc {
    let new = msg![env; this mutableCopy];
    () = msg![env; this sortUsingDescriptors: desc];
    autorelease(env, new)
}

@end

// NSMutableArray is an abstract class. A subclass must provide everything
// NSArray provides, plus:
// - (void)insertObject:(id)object atIndex:(NSUInteger)index;
// - (void)removeObjectAtIndex:(NSUInteger)index;
// - (void)addObject:(id)object;
// - (void)removeLastObject
// - (void)replaceObjectAtIndex:(NSUInteger)index withObject:(id)object;
// Note that it inherits from NSArray, so we must ensure we override any default
// methods that would be inappropriate for mutability.
@implementation NSMutableArray: NSArray

+ (id)allocWithZone:(NSZonePtr)zone {
    // NSArray might be subclassed by something which needs allocWithZone:
    // to have the normal behaviour. Unimplemented: call superclass alloc then.
    assert!(this == env.objc.get_known_class("NSMutableArray", &mut env.mem));
    msg_class![env; _touchHLE_NSMutableArray allocWithZone:zone]
}

+ (id)arrayWithCapacity:(NSUInteger)cap {
    let new = msg![env; this alloc];
    let new = msg![env; new initWithCapacity:cap];
    autorelease(env, new)
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    let new = msg_class![env; NSArray alloc];
    msg![env; new initWithArray: this]
}

 -(())addObjectsFromArray:(id)other {
    let count: NSUInteger = msg![env; other count];
    for i in 0..count {
        let obj: id = msg![env; other objectAtIndex: i];
        () = msg![env; this addObject: obj];
    }
}

@end

// Our private subclass that is the single implementation of NSArray for the
// time being.
@implementation _touchHLE_NSArray: NSArray

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(ArrayHostObject {
        array: Vec::new(),
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    // It seems that every NSArray item in an NSKeyedArchiver plist looks like:
    // {
    //   "$class" => (uid of NSArray class goes here),
    //   "NS.objects" => [
    //     // objects here
    //   ]
    // }
    // Presumably we need to call a `decodeFooBarForKey:` method on the NSCoder
    // here, passing in an NSString for "NS.objects". There is no method for
    // arrays though (maybe it's `decodeObjectForKey:`), and in any case
    // allocating an NSString here would be inconvenient, so let's just take a
    // shortcut.
    // FIXME: What if it's not an NSKeyedUnarchiver?
    let objects = ns_keyed_unarchiver::decode_current_array(env, coder);
    let host_object: &mut ArrayHostObject = env.objc.borrow_mut(this);
    assert!(host_object.array.is_empty());
    host_object.array = objects; // objects are already retained
    this
}

- (())dealloc {
    let host_object: &mut ArrayHostObject = env.objc.borrow_mut(this);
    let array = std::mem::take(&mut host_object.array);

    for object in array {
        release(env, object);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}

- (id)arrayByAddingObject:(NSUInteger)_object {
    msg![env; this init]
}

- (id)boolValue {
    nil
}

// NSFastEnumeration implementation
- (NSUInteger)countByEnumeratingWithState:(MutPtr<NSFastEnumerationState>)state
                                  objects:(MutPtr<id>)stackbuf
                                    count:(NSUInteger)len {
    let mut iterator = env.objc.borrow_mut::<ArrayHostObject>(this).array.iter().copied();
    fast_enumeration_helper(&mut env.mem, this, &mut iterator, state, stackbuf, len)
}

// TODO: more init methods, etc

- (NSUInteger)count {
    env.objc.borrow::<ArrayHostObject>(this).array.len().try_into().unwrap()
}
- (id)objectAtIndex:(NSUInteger)index {
    // TODO: throw real exception rather than panic if out-of-bounds?
    env.objc.borrow::<ArrayHostObject>(this).array[index as usize]
}

// NSFastEnumeration implementation
- (NSUInteger)countByEnumeratingWithState:(MutPtr<NSFastEnumerationState>)state
                                  objects:(MutPtr<id>)stackbuf
                                    count:(NSUInteger)len {
    let host_object = env.objc.borrow::<ArrayHostObject>(this);

    if host_object.array.len() == 0 {
        return 0;
    }

    // TODO: handle size > 1
    // assert!(host_object.array.len() == 1);
    // assert!(len >= host_object.array.len().try_into().unwrap());

    let NSFastEnumerationState {
        state: is_first_round,
        ..
    } = env.mem.read(state);

    match is_first_round {
        0 => {
            let object = host_object.array.iter().next().unwrap();
            env.mem.write(stackbuf, *object);
            env.mem.write(state, NSFastEnumerationState {
                state: 1,
                items_ptr: stackbuf,
                // can be anything as long as it's dereferenceable and the same
                // each iteration
                mutations_ptr: stackbuf.cast(),
                extra: Default::default(),
            });
            1 // returned object count
        },
        1 => {
            0 // end of iteration
        },
        _ => panic!(), // app failed to initialize the buffer?
    }
}

- (())initWithArray:(NSInteger)array copyItems:(bool)_items {
    // TODO
}

- (())writeToFile:(NSInteger)file atomically:(bool)_atomically {
    // TODO
}

@end

@implementation _touchHLE_NSArray_ObjectEnumerator: NSEnumerator

- (id)nextObject {
    let host_obj = env.objc.borrow_mut::<ObjectEnumeratorHostObject>(this);
    host_obj.iterator.next().map_or(nil, |o| o)
}

@end

// Our private subclass that is the single implementation of NSMutableArray for
// the time being.
@implementation _touchHLE_NSMutableArray: NSMutableArray

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(ArrayHostObject {
        array: Vec::new(),
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    let objects = ns_keyed_unarchiver::decode_current_array(env, coder);
    let host_object: &mut ArrayHostObject = env.objc.borrow_mut(this);
    assert!(host_object.array.is_empty());
    host_object.array = objects; // objects are already retained
    this
}

- (())dealloc {
    let host_object: &mut ArrayHostObject = env.objc.borrow_mut(this);
    let array = std::mem::take(&mut host_object.array);

    for object in array {
        release(env, object);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}


// TODO: init methods etc

- (id)initWithCapacity:(NSUInteger)numItems {
    env.objc.borrow_mut::<ArrayHostObject>(this).array.reserve(numItems as usize);
    this
}

- (NSUInteger)count {
    env.objc.borrow::<ArrayHostObject>(this).array.len().try_into().unwrap()
}
- (id)objectAtIndex:(NSUInteger)index {
    // TODO: throw real exception rather than panic if out-of-bounds?
    env.objc.borrow::<ArrayHostObject>(this).array[index as usize]
}

- (id)ascender {
    nil
}

- (id)drain {
    nil
}

- (id)sortUsingSelector {
    nil
}

- (id)reverseObjectEnumerator:(NSUInteger)_object {
    msg![env; this init]
}

- (id)componentsJoinedByString:(NSUInteger)_string {
    msg![env; this init]
}

- (id)sortUsingSelector:(NSUInteger)_using {
    msg![env; this init]
}

- (id)stringByAddingPercentEscapesUsingEncoding:(NSUInteger)_using {
    msg![env; this init]
}

- (id)subarrayWithRange:(NSInteger)_range {
    msg![env; this init]
}

- (id)sortedArrayUsingSelector:(NSUInteger)_selector {
    msg![env; this init]
}

// TODO: more mutation methods

- (())addObject:(id)object {
    retain(env, object);
    env.objc.borrow_mut::<ArrayHostObject>(this).array.push(object);
}

- (())removeObjectAtIndex:(NSUInteger)index {
    let object = env.objc.borrow_mut::<ArrayHostObject>(this).array.remove(index as usize);
    release(env, object)
}

- (())removeLastObject {
    let object = env.objc.borrow_mut::<ArrayHostObject>(this).array.pop().unwrap();
    release(env, object)
}

- (())removeObject:(id)needle {
    let mut objects = mem::take(&mut env.objc.borrow_mut::<ArrayHostObject>(this).array);
    retain(env, needle);
    objects.retain(|&obj| {
        if obj == needle || msg![env; needle isEqual: obj] {
            release(env, obj);
            false
        } else {
            true
        }
    });
    release(env, needle);
    env.objc.borrow_mut::<ArrayHostObject>(this).array = objects;
}

- (())insertObject:(id)obj
           atIndex:(NSUInteger)index {
    let obj = retain(env, obj);
    env.objc.borrow_mut::<ArrayHostObject>(this).array.insert(index as usize, obj);
}

- (())replaceObjectAtIndex:(NSUInteger)index
                withObject:(id)obj {
    let obj = retain(env, obj);
    let old = env.objc.borrow_mut::<ArrayHostObject>(this).array[index as usize];
    env.objc.borrow_mut::<ArrayHostObject>(this).array[index as usize] = obj;
    release(env, old);
}

- (())removeAllObjects {
    let objects = mem::take(&mut env.objc.borrow_mut::<ArrayHostObject>(this).array);
    for object in objects {
        release(env, object);
    }
}

- (())setArray:(bool)array {
    log!("TODO: setArray:{}", array);
}

- (())sortUsingDescriptors:(id)descs {
    let mut v = mem::take(&mut env.objc.borrow_mut::<ArrayHostObject>(this).array);
    v.sort_by(|&a, &b| {
        let mut order = NSOrderedAscending;
        let descs_count: NSUInteger = msg![env; descs count];
        for i in 0..descs_count {
            let desc = msg![env; descs objectAtIndex: i];
            order = msg![env; desc compareObject: a toObject: b];
            if order != 0 {
                break
            }
        }
        match order {
            NSOrderedAscending => Ordering::Less,
            NSOrderedSame => Ordering::Equal,
            NSOrderedDescending => Ordering::Greater,
            _ => panic!(),
        }
    });
    env.objc.borrow_mut::<ArrayHostObject>(this).array = v;
}

- (())writeToFile:(NSInteger)file atomically:(bool)_atomically encoding:(bool)_encoding error:(bool)_error {
    // TODO
}

- (())writeToFile:(NSInteger)file atomically:(bool)_atomically {
    // TODO
}

- (())exchangeObjectAtIndex:(NSInteger)_index withObjectAtIndex:(bool)_object {
    // TODO
}

- (())initWithArray:(NSInteger)array copyItems:(bool)_items {
    // TODO
}

- (())pathForResource:(NSInteger)resource ofType:(bool)_type {
    // TODO
}

@end

// Special variant for use by CFArray with NULL callbacks: objects aren't
// necessarily Objective-C objects and won't be retained/released.
@implementation _touchHLE_NSMutableArray_non_retaining: _touchHLE_NSMutableArray

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem)
}

- (())addObject:(id)object {
    env.objc.borrow_mut::<ArrayHostObject>(this).array.push(object);
}

- (())componentsJoinedByString:(bool)string {
    log!("TODO: componentsJoinedByString:{}", string);
}

- (())removeObjectAtIndex:(NSUInteger)index {
    env.objc.borrow_mut::<ArrayHostObject>(this).array.remove(index as usize);
}

@end

@implementation NSInvocation: NSObject
+ (id)invocationWithMethodSignature:(NSUInteger)_signature {
    msg![env; this init]
}

+ (id)invoke {
    nil
}

+ (())setTarget:(bool)target {
    log!("TODO: setTarget:{}", target);
}

+ (())setSelector:(bool)selector {
    log!("TODO: setSelector:{}", selector);
}

@end

@implementation NSInputStream: NSObject

+ (id)inputStreamWithFileAtPath:(NSUInteger)_path {
    msg![env; this init]
}

+ (id)open {
    nil
}

+ (())read:(NSInteger)read maxLength:(bool)_length {
    // TODO
}

@end

@implementation NSException: NSObject
+ (())raise:(NSInteger)_raise format:(bool)_format {
    // TODO
}

+ (())exceptionWithName:(NSInteger)name reason:(bool)_reason userInfo:(bool)_into {
    // TODO
}

@end

@implementation NSIndexPath: NSObject
@end

@implementation NSNetService: NSObject
- (id)publish {
    nil
}

- (())initWithDomain:(NSInteger)domain type:(bool)_type name:(bool)_name port:(bool)_port {
    // TODO
}

@end

@implementation NSXMLParser: NSObject
- (bool)initWithContentsOfURL:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}
- (id)initWithData:(NSUInteger)_data {
    msg![env; this init]
}

- (id)objectForKey:(id)key {
    let host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let res = host_obj.lookup(env, key);
    *env.objc.borrow_mut(this) = host_obj;
    res
}

- (id)parse {
    nil
}

- (id)parserError {
    nil
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

- (())setShouldResolveExternalEntities:(bool)entities {
    log!("TODO: ShouldResolveExternalEntities:{}", entities);
}

- (())setShouldProcessNamespaces:(bool)process {
    log!("TODO: setShouldProcessNamespaces:{}", process);
}

- (())setShouldReportNamespacePrefixes:(bool)report {
    log!("TODO: setShouldReportNamespacePrefixes:{}", report);
}

@end

@implementation CADisplayLink: NSObject
+ (id)invalidate {
    nil
}

+ (())displayLinkWithTarget:(NSInteger)target selector:(bool)_selector {
    // TODO
}

+ (())setFrameInterval:(bool)frame {
    log!("TODO: setFrameInterval:{}", frame);
}

+ (())addToRunLoop:(NSInteger)_loop forMode:(bool)_mode {
    // TODO
}

@end

@implementation SKProductsRequest: NSObject
@end

};

/// Shortcut for host code, roughly equivalent to
/// `[[NSArray alloc] initWithObjects:count]` but without copying.
/// The elements should already be "retained by" the `Vec`.
pub fn from_vec(env: &mut Environment, objects: Vec<id>) -> id {
    let array: id = msg_class![env; NSMutableArray alloc];
    env.objc.borrow_mut::<ArrayHostObject>(array).array = objects;
    array
}

pub fn to_vec(env: &mut Environment, array: id) -> Vec<id> {
    env.objc.borrow::<ArrayHostObject>(array).array.clone()
}

fn from_va_args(env: &mut Environment, array: id, first: id, rest: DotDotDot) {
    let mut va_args = rest.start();
    let mut v = vec![retain(env, first)];
    loop {
        let obj = va_args.next(env);
        if obj == nil {
            break;
        }
        v.push(retain(env, obj));
    }
    env.objc.borrow_mut::<ArrayHostObject>(array).array = v;
}
