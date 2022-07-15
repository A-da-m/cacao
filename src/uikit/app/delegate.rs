//! This module implements forwarding methods for standard `UIApplicationDelegate` calls. It also
//! creates a custom `UIApplication` subclass that currently does nothing; this is meant as a hook
//! for potential future use.

//use std::ffi::c_void;
use std::sync::Once;
//use std::unreachable;

//use block::Block;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use url::Url;

//use crate::error::Error;
use crate::foundation::{id, nil, NSArray, NSString, NSUInteger, BOOL, NO, YES};
//use crate::user_activity::UserActivity;

use crate::uikit::app::{AppDelegate, APP_DELEGATE};
use crate::uikit::scene::{SceneConfig, SceneConnectionOptions, SceneSession};

#[cfg(feature = "cloudkit")]
use crate::cloudkit::share::CKShareMetaData;

/// A handy method for grabbing our `AppDelegate` from the pointer. This is different from our
/// standard `utils` version as this doesn't require `RefCell` backing.
fn app<T>(this: &Object) -> &T {
    unsafe {
        //let app_ptr: usize = *this.get_ivar(APP_DELEGATE);
        let app = APP_DELEGATE as *const T;
        &*app
    }
}

/// Fires when the Application Delegate receives a `applicationDidFinishLaunching` notification.
extern "C" fn did_finish_launching<T: AppDelegate>(this: &Object, _: Sel, _: id, _: id) -> BOOL {
    app::<T>(this).did_finish_launching();
    YES
}

extern "C" fn configuration_for_scene_session<T: AppDelegate>(this: &Object, _: Sel, _: id, session: id, opts: id) -> id {
    app::<T>(this)
        .config_for_scene_session(SceneSession::with(session), SceneConnectionOptions::with(opts))
        .into_inner()
}

/// Registers an `NSObject` application delegate, and configures it for the various callbacks and
/// pointers we need to have.
pub(crate) fn register_app_delegate_class<T: AppDelegate>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RSTAppDelegate", superclass).unwrap();

        // Launching Applications
        decl.add_method(
            sel!(application:didFinishLaunchingWithOptions:),
            did_finish_launching::<T> as extern "C" fn(&Object, _, _, id) -> BOOL
        );

        // Scenes
        decl.add_method(
            sel!(application:configurationForConnectingSceneSession:options:),
            configuration_for_scene_session::<T> as extern "C" fn(&Object, _, _, id, id) -> id
        );
        /*decl.add_method(
            sel!(application:didDiscardSceneSessions:),
            did_discard_scene_sessions::<T> as extern "C" fn(&Object, _, _, id)
        );*/

        DELEGATE_CLASS = decl.register();
    });

    unsafe { DELEGATE_CLASS }
}
