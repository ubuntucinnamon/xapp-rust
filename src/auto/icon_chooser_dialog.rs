// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::GtkWindow;
use crate::IconSize;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "XAppIconChooserDialog")]
    pub struct IconChooserDialog(Object<ffi::XAppIconChooserDialog, ffi::XAppIconChooserDialogClass>) @extends GtkWindow, gtk::Window, gtk::Widget;

    match fn {
        type_ => || ffi::xapp_icon_chooser_dialog_get_type(),
    }
}

impl IconChooserDialog {
    #[doc(alias = "xapp_icon_chooser_dialog_new")]
    pub fn new() -> IconChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::xapp_icon_chooser_dialog_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`IconChooserDialog`] objects.
            ///
            /// This method returns an instance of [`IconChooserDialogBuilder`](crate::builders::IconChooserDialogBuilder) which can be used to create [`IconChooserDialog`] objects.
            pub fn builder() -> IconChooserDialogBuilder {
                IconChooserDialogBuilder::default()
            }
        

    #[doc(alias = "xapp_icon_chooser_dialog_add_button")]
    pub fn add_button(&self, button: &impl IsA<gtk::Widget>, packing: gtk::PackType, response_id: gtk::ResponseType) {
        unsafe {
            ffi::xapp_icon_chooser_dialog_add_button(self.to_glib_none().0, button.as_ref().to_glib_none().0, packing.into_glib(), response_id.into_glib());
        }
    }

    #[doc(alias = "xapp_icon_chooser_dialog_add_custom_category")]
    pub fn add_custom_category(&self, name: &str, icons: &[&str]) {
        unsafe {
            ffi::xapp_icon_chooser_dialog_add_custom_category(self.to_glib_none().0, name.to_glib_none().0, icons.to_glib_full());
        }
    }

    #[doc(alias = "xapp_icon_chooser_dialog_get_default_icon")]
    #[doc(alias = "get_default_icon")]
    pub fn default_icon(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_icon_chooser_dialog_get_default_icon(self.to_glib_none().0))
        }
    }

    #[doc(alias = "xapp_icon_chooser_dialog_get_icon_string")]
    #[doc(alias = "get_icon_string")]
    pub fn icon_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_icon_chooser_dialog_get_icon_string(self.to_glib_none().0))
        }
    }

    #[doc(alias = "xapp_icon_chooser_dialog_run")]
    pub fn run(&self) -> i32 {
        unsafe {
            ffi::xapp_icon_chooser_dialog_run(self.to_glib_none().0)
        }
    }

    #[doc(alias = "xapp_icon_chooser_dialog_run_with_category")]
    pub fn run_with_category(&self, category: &str) -> i32 {
        unsafe {
            ffi::xapp_icon_chooser_dialog_run_with_category(self.to_glib_none().0, category.to_glib_none().0)
        }
    }

    #[doc(alias = "xapp_icon_chooser_dialog_run_with_icon")]
    pub fn run_with_icon(&self, icon: &str) -> i32 {
        unsafe {
            ffi::xapp_icon_chooser_dialog_run_with_icon(self.to_glib_none().0, icon.to_glib_none().0)
        }
    }

    #[doc(alias = "xapp_icon_chooser_dialog_set_default_icon")]
    pub fn set_default_icon(&self, icon: &str) {
        unsafe {
            ffi::xapp_icon_chooser_dialog_set_default_icon(self.to_glib_none().0, icon.to_glib_none().0);
        }
    }

    #[doc(alias = "allow-paths")]
    pub fn allows_paths(&self) -> bool {
        glib::ObjectExt::property(self, "allow-paths")
    }

    #[doc(alias = "allow-paths")]
    pub fn set_allow_paths(&self, allow_paths: bool) {
        glib::ObjectExt::set_property(self,"allow-paths", &allow_paths)
    }

    #[doc(alias = "icon-size")]
    pub fn icon_size(&self) -> IconSize {
        glib::ObjectExt::property(self, "icon-size")
    }

    #[doc(alias = "icon-size")]
    pub fn set_icon_size(&self, icon_size: IconSize) {
        glib::ObjectExt::set_property(self,"icon-size", &icon_size)
    }

    #[doc(alias = "close")]
    pub fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<F: Fn(&IconChooserDialog) + 'static>(this: *mut ffi::XAppIconChooserDialog, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(close_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    pub fn emit_close(&self) {
        self.emit_by_name::<()>("close", &[]);
    }

    #[doc(alias = "select")]
    pub fn connect_select<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn select_trampoline<F: Fn(&IconChooserDialog) + 'static>(this: *mut ffi::XAppIconChooserDialog, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"select\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(select_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    pub fn emit_select(&self) {
        self.emit_by_name::<()>("select", &[]);
    }

    #[doc(alias = "allow-paths")]
    pub fn connect_allow_paths_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_paths_trampoline<F: Fn(&IconChooserDialog) + 'static>(this: *mut ffi::XAppIconChooserDialog, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::allow-paths\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_allow_paths_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "default-icon")]
    pub fn connect_default_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_icon_trampoline<F: Fn(&IconChooserDialog) + 'static>(this: *mut ffi::XAppIconChooserDialog, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::default-icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_default_icon_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon-size")]
    pub fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<F: Fn(&IconChooserDialog) + 'static>(this: *mut ffi::XAppIconChooserDialog, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_icon_size_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for IconChooserDialog {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`IconChooserDialog`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct IconChooserDialogBuilder {
    allow_paths: Option<bool>,
    default_icon: Option<String>,
    icon_size: Option<IconSize>,
    #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_4")))]
    accept_focus: Option<bool>,
    //application: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v3_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_4")))]
    attached_to: Option<gtk::Widget>,
    #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_4")))]
    decorated: Option<bool>,
    default_height: Option<i32>,
    default_width: Option<i32>,
    #[cfg(any(feature = "gtk_v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_10")))]
    deletable: Option<bool>,
    destroy_with_parent: Option<bool>,
    #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_6")))]
    focus_on_map: Option<bool>,
    #[cfg(any(feature = "gtk_v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_20")))]
    focus_visible: Option<bool>,
    //gravity: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    has_resize_grip: Option<bool>,
    #[cfg(any(feature = "gtk_v3_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_4")))]
    hide_titlebar_when_maximized: Option<bool>,
    //icon: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_6")))]
    icon_name: Option<String>,
    #[cfg(any(feature = "gtk_v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_20")))]
    mnemonics_visible: Option<bool>,
    modal: Option<bool>,
    resizable: Option<bool>,
    role: Option<String>,
    //screen: /*Unknown type*/,
    skip_pager_hint: Option<bool>,
    skip_taskbar_hint: Option<bool>,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    startup_id: Option<String>,
    title: Option<String>,
    #[cfg(any(feature = "gtk_v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_10")))]
    transient_for: Option<gtk::Window>,
    type_: Option<gtk::WindowType>,
    //type-hint: /*Unknown type*/,
    urgency_hint: Option<bool>,
    //window-position: /*Unknown type*/,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    #[cfg(any(feature = "gtk_v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_18")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    double_buffered: Option<bool>,
    //events: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    expand: Option<bool>,
    #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_20")))]
    focus_on_click: Option<bool>,
    //halign: /*Unknown type*/,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    hexpand: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin_bottom: Option<i32>,
    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    margin_end: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    margin_left: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    margin_right: Option<i32>,
    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    margin_start: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_8")))]
    opacity: Option<f64>,
    //parent: /*Unknown type*/,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    tooltip_markup: Option<String>,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    tooltip_text: Option<String>,
    //valign: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    vexpand: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl IconChooserDialogBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`IconChooserDialogBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`IconChooserDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> IconChooserDialog {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
if let Some(ref allow_paths) = self.allow_paths {
                properties.push(("allow-paths", allow_paths));
            }
if let Some(ref default_icon) = self.default_icon {
                properties.push(("default-icon", default_icon));
            }
if let Some(ref icon_size) = self.icon_size {
                properties.push(("icon-size", icon_size));
            }
        #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
if let Some(ref accept_focus) = self.accept_focus {
                properties.push(("accept-focus", accept_focus));
            }
        #[cfg(any(feature = "gtk_v3_4", feature = "dox"))]
if let Some(ref attached_to) = self.attached_to {
                properties.push(("attached-to", attached_to));
            }
        #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
if let Some(ref decorated) = self.decorated {
                properties.push(("decorated", decorated));
            }
if let Some(ref default_height) = self.default_height {
                properties.push(("default-height", default_height));
            }
if let Some(ref default_width) = self.default_width {
                properties.push(("default-width", default_width));
            }
        #[cfg(any(feature = "gtk_v2_10", feature = "dox"))]
if let Some(ref deletable) = self.deletable {
                properties.push(("deletable", deletable));
            }
if let Some(ref destroy_with_parent) = self.destroy_with_parent {
                properties.push(("destroy-with-parent", destroy_with_parent));
            }
        #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
if let Some(ref focus_on_map) = self.focus_on_map {
                properties.push(("focus-on-map", focus_on_map));
            }
        #[cfg(any(feature = "gtk_v2_20", feature = "dox"))]
if let Some(ref focus_visible) = self.focus_visible {
                properties.push(("focus-visible", focus_visible));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref has_resize_grip) = self.has_resize_grip {
                properties.push(("has-resize-grip", has_resize_grip));
            }
        #[cfg(any(feature = "gtk_v3_4", feature = "dox"))]
if let Some(ref hide_titlebar_when_maximized) = self.hide_titlebar_when_maximized {
                properties.push(("hide-titlebar-when-maximized", hide_titlebar_when_maximized));
            }
        #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
if let Some(ref icon_name) = self.icon_name {
                properties.push(("icon-name", icon_name));
            }
        #[cfg(any(feature = "gtk_v2_20", feature = "dox"))]
if let Some(ref mnemonics_visible) = self.mnemonics_visible {
                properties.push(("mnemonics-visible", mnemonics_visible));
            }
if let Some(ref modal) = self.modal {
                properties.push(("modal", modal));
            }
if let Some(ref resizable) = self.resizable {
                properties.push(("resizable", resizable));
            }
if let Some(ref role) = self.role {
                properties.push(("role", role));
            }
if let Some(ref skip_pager_hint) = self.skip_pager_hint {
                properties.push(("skip-pager-hint", skip_pager_hint));
            }
if let Some(ref skip_taskbar_hint) = self.skip_taskbar_hint {
                properties.push(("skip-taskbar-hint", skip_taskbar_hint));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref startup_id) = self.startup_id {
                properties.push(("startup-id", startup_id));
            }
if let Some(ref title) = self.title {
                properties.push(("title", title));
            }
        #[cfg(any(feature = "gtk_v2_10", feature = "dox"))]
if let Some(ref transient_for) = self.transient_for {
                properties.push(("transient-for", transient_for));
            }
if let Some(ref type_) = self.type_ {
                properties.push(("type", type_));
            }
if let Some(ref urgency_hint) = self.urgency_hint {
                properties.push(("urgency-hint", urgency_hint));
            }
if let Some(ref app_paintable) = self.app_paintable {
                properties.push(("app-paintable", app_paintable));
            }
if let Some(ref can_default) = self.can_default {
                properties.push(("can-default", can_default));
            }
if let Some(ref can_focus) = self.can_focus {
                properties.push(("can-focus", can_focus));
            }
        #[cfg(any(feature = "gtk_v2_18", feature = "dox"))]
if let Some(ref double_buffered) = self.double_buffered {
                properties.push(("double-buffered", double_buffered));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref expand) = self.expand {
                properties.push(("expand", expand));
            }
        #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
if let Some(ref has_default) = self.has_default {
                properties.push(("has-default", has_default));
            }
if let Some(ref has_focus) = self.has_focus {
                properties.push(("has-focus", has_focus));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref has_tooltip) = self.has_tooltip {
                properties.push(("has-tooltip", has_tooltip));
            }
if let Some(ref height_request) = self.height_request {
                properties.push(("height-request", height_request));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref hexpand) = self.hexpand {
                properties.push(("hexpand", hexpand));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref hexpand_set) = self.hexpand_set {
                properties.push(("hexpand-set", hexpand_set));
            }
if let Some(ref is_focus) = self.is_focus {
                properties.push(("is-focus", is_focus));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin) = self.margin {
                properties.push(("margin", margin));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_bottom) = self.margin_bottom {
                properties.push(("margin-bottom", margin_bottom));
            }
        #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
if let Some(ref margin_end) = self.margin_end {
                properties.push(("margin-end", margin_end));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_left) = self.margin_left {
                properties.push(("margin-left", margin_left));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_right) = self.margin_right {
                properties.push(("margin-right", margin_right));
            }
        #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
if let Some(ref margin_start) = self.margin_start {
                properties.push(("margin-start", margin_start));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_top) = self.margin_top {
                properties.push(("margin-top", margin_top));
            }
if let Some(ref name) = self.name {
                properties.push(("name", name));
            }
if let Some(ref no_show_all) = self.no_show_all {
                properties.push(("no-show-all", no_show_all));
            }
        #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
if let Some(ref opacity) = self.opacity {
                properties.push(("opacity", opacity));
            }
if let Some(ref receives_default) = self.receives_default {
                properties.push(("receives-default", receives_default));
            }
if let Some(ref sensitive) = self.sensitive {
                properties.push(("sensitive", sensitive));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref tooltip_markup) = self.tooltip_markup {
                properties.push(("tooltip-markup", tooltip_markup));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref tooltip_text) = self.tooltip_text {
                properties.push(("tooltip-text", tooltip_text));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref vexpand) = self.vexpand {
                properties.push(("vexpand", vexpand));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref vexpand_set) = self.vexpand_set {
                properties.push(("vexpand-set", vexpand_set));
            }
if let Some(ref visible) = self.visible {
                properties.push(("visible", visible));
            }
if let Some(ref width_request) = self.width_request {
                properties.push(("width-request", width_request));
            }
        glib::Object::new::<IconChooserDialog>(&properties)

    }

    pub fn allow_paths(mut self, allow_paths: bool) -> Self {
        self.allow_paths = Some(allow_paths);
        self
    }

    pub fn default_icon(mut self, default_icon: &str) -> Self {
        self.default_icon = Some(default_icon.to_string());
        self
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_4")))]
    pub fn accept_focus(mut self, accept_focus: bool) -> Self {
        self.accept_focus = Some(accept_focus);
        self
    }

    #[cfg(any(feature = "gtk_v3_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_4")))]
    pub fn attached_to(mut self, attached_to: &impl IsA<gtk::Widget>) -> Self {
        self.attached_to = Some(attached_to.clone().upcast());
        self
    }

    #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_4")))]
    pub fn decorated(mut self, decorated: bool) -> Self {
        self.decorated = Some(decorated);
        self
    }

    pub fn default_height(mut self, default_height: i32) -> Self {
        self.default_height = Some(default_height);
        self
    }

    pub fn default_width(mut self, default_width: i32) -> Self {
        self.default_width = Some(default_width);
        self
    }

    #[cfg(any(feature = "gtk_v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_10")))]
    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = Some(deletable);
        self
    }

    pub fn destroy_with_parent(mut self, destroy_with_parent: bool) -> Self {
        self.destroy_with_parent = Some(destroy_with_parent);
        self
    }

    #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_6")))]
    pub fn focus_on_map(mut self, focus_on_map: bool) -> Self {
        self.focus_on_map = Some(focus_on_map);
        self
    }

    #[cfg(any(feature = "gtk_v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_20")))]
    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.focus_visible = Some(focus_visible);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn has_resize_grip(mut self, has_resize_grip: bool) -> Self {
        self.has_resize_grip = Some(has_resize_grip);
        self
    }

    #[cfg(any(feature = "gtk_v3_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_4")))]
    pub fn hide_titlebar_when_maximized(mut self, hide_titlebar_when_maximized: bool) -> Self {
        self.hide_titlebar_when_maximized = Some(hide_titlebar_when_maximized);
        self
    }

    #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_6")))]
    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_20")))]
    pub fn mnemonics_visible(mut self, mnemonics_visible: bool) -> Self {
        self.mnemonics_visible = Some(mnemonics_visible);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn role(mut self, role: &str) -> Self {
        self.role = Some(role.to_string());
        self
    }

    pub fn skip_pager_hint(mut self, skip_pager_hint: bool) -> Self {
        self.skip_pager_hint = Some(skip_pager_hint);
        self
    }

    pub fn skip_taskbar_hint(mut self, skip_taskbar_hint: bool) -> Self {
        self.skip_taskbar_hint = Some(skip_taskbar_hint);
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn startup_id(mut self, startup_id: &str) -> Self {
        self.startup_id = Some(startup_id.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_10")))]
    pub fn transient_for(mut self, transient_for: &impl IsA<gtk::Window>) -> Self {
        self.transient_for = Some(transient_for.clone().upcast());
        self
    }

    pub fn type_(mut self, type_: gtk::WindowType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn urgency_hint(mut self, urgency_hint: bool) -> Self {
        self.urgency_hint = Some(urgency_hint);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    #[cfg(any(feature = "gtk_v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_18")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn double_buffered(mut self, double_buffered: bool) -> Self {
        self.double_buffered = Some(double_buffered);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_left(mut self, margin_left: i32) -> Self {
        self.margin_left = Some(margin_left);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_right(mut self, margin_right: i32) -> Self {
        self.margin_right = Some(margin_right);
        self
    }

    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_8")))]
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

impl fmt::Display for IconChooserDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IconChooserDialog")
    }
}
