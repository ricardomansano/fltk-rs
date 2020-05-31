/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
pub type custom_handler_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type custom_draw_callback =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Widget_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Widget_x(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_y(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_width(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_height(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_label(arg1: *mut Fl_Widget) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Widget_set_label(arg1: *mut Fl_Widget, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Widget_redraw(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_show(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_hide(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_activate(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_deactivate(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_redraw_label(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_resize(
        arg1: *mut Fl_Widget,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Widget_tooltip(arg1: *mut Fl_Widget) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Widget_set_tooltip(arg1: *mut Fl_Widget, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Widget_get_type(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_type(arg1: *mut Fl_Widget, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_color(arg1: *mut Fl_Widget) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_color(arg1: *mut Fl_Widget, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Widget_label_color(arg1: *mut Fl_Widget) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_label_color(arg1: *mut Fl_Widget, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Widget_label_font(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_font(arg1: *mut Fl_Widget, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_label_size(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_size(arg1: *mut Fl_Widget, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_label_type(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_type(arg1: *mut Fl_Widget, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_box(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_box(arg1: *mut Fl_Widget, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_changed(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_changed(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_clear_changed(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_align(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_align(arg1: *mut Fl_Widget, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_delete(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_set_image(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Widget_set_image_with_size(
        arg1: *mut Fl_Widget,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Widget_set_handler(
        self_: *mut Fl_Widget,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Widget_set_draw(
        self_: *mut Fl_Widget,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Widget_set_trigger(arg1: *mut Fl_Widget, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_image(arg1: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_parent(self_: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_selection_color(arg1: *mut Fl_Widget) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_selection_color(arg1: *mut Fl_Widget, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Widget_do_callback(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_inside(
        self_: *const Fl_Widget,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_window(arg1: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_top_window(arg1: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_takes_events(arg1: *const Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_user_data(arg1: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_take_focus(self_: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_visible_focus(self_: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_clear_visible_focus(self_: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_visible_focus(self_: *mut Fl_Widget, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_has_visible_focus(self_: *mut Fl_Widget) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_user_data(arg1: *mut Fl_Widget, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Widget_draw_data(self_: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_handle_data(self_: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_set_draw_data(self_: *mut Fl_Widget, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Widget_set_handle_data(self_: *mut Fl_Widget, data: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Spinner {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Spinner_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Spinner;
}
extern "C" {
    pub fn Fl_Spinner_x(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_y(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_width(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_height(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_label(arg1: *mut Fl_Spinner) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Spinner_set_label(arg1: *mut Fl_Spinner, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Spinner_redraw(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_show(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_hide(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_activate(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_deactivate(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_redraw_label(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_resize(
        arg1: *mut Fl_Spinner,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Spinner_tooltip(arg1: *mut Fl_Spinner) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Spinner_set_tooltip(arg1: *mut Fl_Spinner, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Spinner_get_type(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_type(arg1: *mut Fl_Spinner, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_color(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Spinner_set_color(arg1: *mut Fl_Spinner, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Spinner_label_color(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Spinner_set_label_color(arg1: *mut Fl_Spinner, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Spinner_label_font(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_label_font(arg1: *mut Fl_Spinner, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_label_size(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_label_size(arg1: *mut Fl_Spinner, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_label_type(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_label_type(arg1: *mut Fl_Spinner, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_box(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_box(arg1: *mut Fl_Spinner, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_changed(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_changed(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_clear_changed(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_align(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_align(arg1: *mut Fl_Spinner, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_delete(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_set_image(arg1: *mut Fl_Spinner, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Spinner_set_image_with_size(
        arg1: *mut Fl_Spinner,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Spinner_set_handler(
        self_: *mut Fl_Spinner,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Spinner_set_draw(
        self_: *mut Fl_Spinner,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Spinner_set_trigger(arg1: *mut Fl_Spinner, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_image(arg1: *const Fl_Spinner) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Spinner_parent(self_: *const Fl_Spinner) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Spinner_selection_color(arg1: *mut Fl_Spinner) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Spinner_set_selection_color(arg1: *mut Fl_Spinner, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Spinner_do_callback(arg1: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_inside(
        self_: *const Fl_Spinner,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_window(arg1: *const Fl_Spinner) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Spinner_top_window(arg1: *const Fl_Spinner) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Spinner_takes_events(arg1: *const Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_user_data(arg1: *const Fl_Spinner) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Spinner_take_focus(self_: *mut Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_visible_focus(self_: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_clear_visible_focus(self_: *mut Fl_Spinner);
}
extern "C" {
    pub fn Fl_Spinner_visible_focus(self_: *mut Fl_Spinner, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_has_visible_focus(self_: *mut Fl_Spinner) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Spinner_set_user_data(arg1: *mut Fl_Spinner, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Spinner_draw_data(self_: *const Fl_Spinner) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Spinner_handle_data(self_: *const Fl_Spinner) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Spinner_set_draw_data(self_: *mut Fl_Spinner, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Spinner_set_handle_data(self_: *mut Fl_Spinner, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Spinner_minimum(arg1: *mut Fl_Spinner) -> f64;
}
extern "C" {
    pub fn Fl_Spinner_set_minimum(arg1: *mut Fl_Spinner, a: f64);
}
extern "C" {
    pub fn Fl_Spinner_maximum(arg1: *mut Fl_Spinner) -> f64;
}
extern "C" {
    pub fn Fl_Spinner_set_maximum(arg1: *mut Fl_Spinner, a: f64);
}
extern "C" {
    pub fn Fl_Spinner_set_range(arg1: *mut Fl_Spinner, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Spinner_set_step(arg1: *mut Fl_Spinner, a: f64);
}
extern "C" {
    pub fn Fl_Spinner_step(arg1: *mut Fl_Spinner) -> f64;
}
extern "C" {
    pub fn Fl_Spinner_maxsize(self_: *const Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_maxsize(self_: *mut Fl_Spinner, m: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_text_font(self_: *const Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_text_font(self_: *mut Fl_Spinner, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_text_size(self_: *const Fl_Spinner) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Spinner_set_textsize(self_: *mut Fl_Spinner, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Spinner_text_color(self_: *const Fl_Spinner) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Spinner_set_text_color(self_: *mut Fl_Spinner, n: ::std::os::raw::c_uint);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Clock {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Clock_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Clock;
}
extern "C" {
    pub fn Fl_Clock_x(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_y(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_width(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_height(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_label(arg1: *mut Fl_Clock) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Clock_set_label(arg1: *mut Fl_Clock, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Clock_redraw(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_show(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_hide(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_activate(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_deactivate(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_redraw_label(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_resize(
        arg1: *mut Fl_Clock,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Clock_tooltip(arg1: *mut Fl_Clock) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Clock_set_tooltip(arg1: *mut Fl_Clock, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Clock_get_type(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_set_type(arg1: *mut Fl_Clock, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Clock_color(arg1: *mut Fl_Clock) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Clock_set_color(arg1: *mut Fl_Clock, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Clock_label_color(arg1: *mut Fl_Clock) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Clock_set_label_color(arg1: *mut Fl_Clock, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Clock_label_font(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_set_label_font(arg1: *mut Fl_Clock, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Clock_label_size(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_set_label_size(arg1: *mut Fl_Clock, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Clock_label_type(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_set_label_type(arg1: *mut Fl_Clock, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Clock_box(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_set_box(arg1: *mut Fl_Clock, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Clock_changed(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_set_changed(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_clear_changed(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_align(arg1: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_set_align(arg1: *mut Fl_Clock, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Clock_delete(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_set_image(arg1: *mut Fl_Clock, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Clock_set_image_with_size(
        arg1: *mut Fl_Clock,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Clock_set_handler(
        self_: *mut Fl_Clock,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Clock_set_draw(
        self_: *mut Fl_Clock,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Clock_set_trigger(arg1: *mut Fl_Clock, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Clock_image(arg1: *const Fl_Clock) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Clock_parent(self_: *const Fl_Clock) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Clock_selection_color(arg1: *mut Fl_Clock) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Clock_set_selection_color(arg1: *mut Fl_Clock, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Clock_do_callback(arg1: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_inside(
        self_: *const Fl_Clock,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_window(arg1: *const Fl_Clock) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Clock_top_window(arg1: *const Fl_Clock) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Clock_takes_events(arg1: *const Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_user_data(arg1: *const Fl_Clock) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Clock_take_focus(self_: *mut Fl_Clock) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Clock_set_visible_focus(self_: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_clear_visible_focus(self_: *mut Fl_Clock);
}
extern "C" {
    pub fn Fl_Clock_visible_focus(self_: *mut Fl_Clock, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Clock_has_visible_focus(self_: *mut Fl_Clock) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Clock_set_user_data(arg1: *mut Fl_Clock, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Clock_draw_data(self_: *const Fl_Clock) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Clock_handle_data(self_: *const Fl_Clock) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Clock_set_draw_data(self_: *mut Fl_Clock, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Clock_set_handle_data(self_: *mut Fl_Clock, data: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Chart {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Chart_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Chart;
}
extern "C" {
    pub fn Fl_Chart_x(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_y(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_width(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_height(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_label(arg1: *mut Fl_Chart) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Chart_set_label(arg1: *mut Fl_Chart, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Chart_redraw(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_show(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_hide(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_activate(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_deactivate(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_redraw_label(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_resize(
        arg1: *mut Fl_Chart,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Chart_tooltip(arg1: *mut Fl_Chart) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Chart_set_tooltip(arg1: *mut Fl_Chart, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Chart_get_type(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_type(arg1: *mut Fl_Chart, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_color(arg1: *mut Fl_Chart) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Chart_set_color(arg1: *mut Fl_Chart, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Chart_label_color(arg1: *mut Fl_Chart) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Chart_set_label_color(arg1: *mut Fl_Chart, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Chart_label_font(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_label_font(arg1: *mut Fl_Chart, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_label_size(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_label_size(arg1: *mut Fl_Chart, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_label_type(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_label_type(arg1: *mut Fl_Chart, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_box(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_box(arg1: *mut Fl_Chart, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_changed(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_changed(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_clear_changed(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_align(arg1: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_align(arg1: *mut Fl_Chart, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_delete(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_set_image(arg1: *mut Fl_Chart, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Chart_set_image_with_size(
        arg1: *mut Fl_Chart,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Chart_set_handler(
        self_: *mut Fl_Chart,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Chart_set_draw(
        self_: *mut Fl_Chart,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Chart_set_trigger(arg1: *mut Fl_Chart, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_image(arg1: *const Fl_Chart) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Chart_parent(self_: *const Fl_Chart) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Chart_selection_color(arg1: *mut Fl_Chart) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Chart_set_selection_color(arg1: *mut Fl_Chart, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Chart_do_callback(arg1: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_inside(
        self_: *const Fl_Chart,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_window(arg1: *const Fl_Chart) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Chart_top_window(arg1: *const Fl_Chart) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Chart_takes_events(arg1: *const Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_user_data(arg1: *const Fl_Chart) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Chart_take_focus(self_: *mut Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_visible_focus(self_: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_clear_visible_focus(self_: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_visible_focus(self_: *mut Fl_Chart, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_has_visible_focus(self_: *mut Fl_Chart) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Chart_set_user_data(arg1: *mut Fl_Chart, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Chart_draw_data(self_: *const Fl_Chart) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Chart_handle_data(self_: *const Fl_Chart) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Chart_set_draw_data(self_: *mut Fl_Chart, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Chart_set_handle_data(self_: *mut Fl_Chart, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Chart_clear(self_: *mut Fl_Chart);
}
extern "C" {
    pub fn Fl_Chart_add(
        self_: *mut Fl_Chart,
        val: f64,
        str: *const ::std::os::raw::c_char,
        col: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Chart_insert(
        self_: *mut Fl_Chart,
        ind: ::std::os::raw::c_int,
        val: f64,
        str: *const ::std::os::raw::c_char,
        col: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Chart_replace(
        self_: *mut Fl_Chart,
        ind: ::std::os::raw::c_int,
        val: f64,
        str: *const ::std::os::raw::c_char,
        col: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Chart_set_bounds(self_: *mut Fl_Chart, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Chart_size(self_: *const Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_size(
        self_: *mut Fl_Chart,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Chart_maxsize(self_: *const Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_maxsize(self_: *mut Fl_Chart, m: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_text_font(self_: *const Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_text_font(self_: *mut Fl_Chart, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_text_size(self_: *const Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_set_textsize(self_: *mut Fl_Chart, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Chart_text_color(self_: *const Fl_Chart) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Chart_set_text_color(self_: *mut Fl_Chart, n: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Chart_is_autosize(self_: *const Fl_Chart) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Chart_make_autosize(self_: *mut Fl_Chart, n: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Progress {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Progress_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Progress;
}
extern "C" {
    pub fn Fl_Progress_x(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_y(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_width(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_height(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_label(arg1: *mut Fl_Progress) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Progress_set_label(arg1: *mut Fl_Progress, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Progress_redraw(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_show(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_hide(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_activate(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_deactivate(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_redraw_label(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_resize(
        arg1: *mut Fl_Progress,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Progress_tooltip(arg1: *mut Fl_Progress) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Progress_set_tooltip(arg1: *mut Fl_Progress, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Progress_get_type(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_set_type(arg1: *mut Fl_Progress, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Progress_color(arg1: *mut Fl_Progress) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Progress_set_color(arg1: *mut Fl_Progress, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Progress_label_color(arg1: *mut Fl_Progress) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Progress_set_label_color(arg1: *mut Fl_Progress, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Progress_label_font(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_set_label_font(arg1: *mut Fl_Progress, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Progress_label_size(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_set_label_size(arg1: *mut Fl_Progress, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Progress_label_type(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_set_label_type(arg1: *mut Fl_Progress, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Progress_box(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_set_box(arg1: *mut Fl_Progress, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Progress_changed(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_set_changed(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_clear_changed(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_align(arg1: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_set_align(arg1: *mut Fl_Progress, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Progress_delete(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_set_image(arg1: *mut Fl_Progress, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Progress_set_image_with_size(
        arg1: *mut Fl_Progress,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Progress_set_handler(
        self_: *mut Fl_Progress,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Progress_set_draw(
        self_: *mut Fl_Progress,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Progress_set_trigger(arg1: *mut Fl_Progress, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Progress_image(arg1: *const Fl_Progress) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Progress_parent(self_: *const Fl_Progress) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Progress_selection_color(arg1: *mut Fl_Progress) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Progress_set_selection_color(arg1: *mut Fl_Progress, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Progress_do_callback(arg1: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_inside(
        self_: *const Fl_Progress,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_window(arg1: *const Fl_Progress) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Progress_top_window(arg1: *const Fl_Progress) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Progress_takes_events(arg1: *const Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_user_data(arg1: *const Fl_Progress) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Progress_take_focus(self_: *mut Fl_Progress) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Progress_set_visible_focus(self_: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_clear_visible_focus(self_: *mut Fl_Progress);
}
extern "C" {
    pub fn Fl_Progress_visible_focus(self_: *mut Fl_Progress, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Progress_has_visible_focus(self_: *mut Fl_Progress) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Progress_set_user_data(arg1: *mut Fl_Progress, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Progress_draw_data(self_: *const Fl_Progress) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Progress_handle_data(self_: *const Fl_Progress) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Progress_set_draw_data(self_: *mut Fl_Progress, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Progress_set_handle_data(self_: *mut Fl_Progress, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Progress_minimum(arg1: *mut Fl_Progress) -> f64;
}
extern "C" {
    pub fn Fl_Progress_set_minimum(arg1: *mut Fl_Progress, a: f64);
}
extern "C" {
    pub fn Fl_Progress_maximum(arg1: *mut Fl_Progress) -> f64;
}
extern "C" {
    pub fn Fl_Progress_set_maximum(arg1: *mut Fl_Progress, a: f64);
}
extern "C" {
    pub fn Fl_Progress_value(arg1: *mut Fl_Progress) -> f64;
}
extern "C" {
    pub fn Fl_Progress_set_value(arg1: *mut Fl_Progress, arg2: f64);
}
extern "C" {
    pub fn Fl_Tooltip_delay() -> f32;
}
extern "C" {
    pub fn Fl_Tooltip_set_delay(f: f32);
}
extern "C" {
    pub fn Fl_Tooltip_hidedelay() -> f32;
}
extern "C" {
    pub fn Fl_Tooltip_set_hidedelay(f: f32);
}
extern "C" {
    pub fn Fl_Tooltip_hoverdelay() -> f32;
}
extern "C" {
    pub fn Fl_Tooltip_set_hoverdelay(f: f32);
}
extern "C" {
    pub fn Fl_Tooltip_enabled() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tooltip_enable(b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tooltip_disable();
}
extern "C" {
    pub static mut Fl_Tooltip_enter: ::std::option::Option<unsafe extern "C" fn(w: *mut Fl_Widget)>;
}
extern "C" {
    pub fn Fl_Tooltip_enter_area(
        w: *mut Fl_Widget,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        tip: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub static mut Fl_Tooltip_exit: ::std::option::Option<unsafe extern "C" fn(w: *mut Fl_Widget)>;
}
extern "C" {
    pub fn Fl_Tooltip_current_widget() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tooltip_current(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Tooltip_font() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tooltip_set_font(i: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tooltip_font_size() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tooltip_set_font_size(s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tooltip_color() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tooltip_set_color(c: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tooltip_text_color() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tooltip_set_text_color(c: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tooltip_margin_width() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tooltip_set_margin_width(v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tooltip_margin_height() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tooltip_set_margin_height(v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tooltip_wrap_width() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tooltip_set_wrap_width(v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tooltip_current_window() -> *mut ::std::os::raw::c_void;
}
